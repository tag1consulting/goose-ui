use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

mod services;
use crate::services::websocket::WebsocketService;
use crate::services::event_bus::EventBus;

enum Msg {
    Start,
    Stop,
    HandleMsg(String),
}

struct Controls {
    running: bool,
    status: &'static str,
    wss: WebsocketService,
    _producer: Box<dyn Bridge<EventBus>>,
}

impl Component for Controls {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            running: false,
            status: "Connected",
            wss: WebsocketService::new(),
            _producer: EventBus::bridge(ctx.link().callback(Msg::HandleMsg))
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Start => {
                if self.running {
                    return false;
                } 
                self.running = true;
                log::debug!("Start requested...");
                self.wss.start();
                true
            }
            Msg::Stop => {
                if !self.running {
                    return false;
                } 
                self.running = false;
                log::debug!("Stop requested...");
                self.wss.stop();
                true
            }
            Msg::HandleMsg(s) => {
                match s.as_str() {
                    "started" => {
                        self.status = "Running";
                    }
                    "stopped" => {
                        self.status = "Stopped";
                    }
                    _ => {
                        ();
                    }
                };
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <div>
                <div>
                    <button onclick={link.callback(|_| Msg::Start)}>{ "Start" }</button>
                    <button onclick={link.callback(|_| Msg::Stop)}>{ "Stop" }</button>
                </div>
                <div>
                    { "Status: " } { self.status }
                </div>
            </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Controls>();
}
