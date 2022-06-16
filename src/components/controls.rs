use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::components::main::Msg as MainMsg;
use crate::services::event_bus::EventBus;

pub enum Msg {
    Start,
    Stop,
    HandleMsg(String),
}

pub struct Controls {
    running: bool,
    status: &'static str,
    _producer: Box<dyn Bridge<EventBus>>,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub parent_cb: Callback<MainMsg>,
    pub connected: bool,
}

impl Component for Controls {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            running: false,
            status: "Disconnected",
            _producer: EventBus::bridge(ctx.link().callback(Msg::HandleMsg)),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Start => {
                if self.running {
                    return false;
                }
                self.running = true;
                log::debug!("Start requested...");
                ctx.props().parent_cb.emit(MainMsg::Start);
                true
            }
            Msg::Stop => {
                if !self.running {
                    return false;
                }
                self.running = false;
                log::debug!("Stop requested...");
                ctx.props().parent_cb.emit(MainMsg::Stop);
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

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        if ctx.props().connected && self.status == "Disconnected" {
            self.status = "Connected";
            return true;
        } else if !ctx.props().connected {
            self.status = "Disconnected";
            return true;
        }

        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <div>
                <div>
                    <button disabled={ !ctx.props().connected } onclick={link.callback(|_| Msg::Start)}>{ "Start" }</button>
                    <button disabled={ !ctx.props().connected } onclick={link.callback(|_| Msg::Stop)}>{ "Stop" }</button>
                </div>
                <div>
                    { "Status: " } { self.status }
                </div>
            </div>
        }
    }
}
