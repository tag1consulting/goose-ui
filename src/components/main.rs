use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::components::connector::Connector;
use crate::components::controls::Controls;
use crate::services::event_bus::EventBus;
use crate::services::websocket::WebsocketService;

pub enum Msg {
    Connect,
    Disconnect,
    Start,
    Stop,
    HandleMsg(String),
}

pub struct Main {
    wss: Option<WebsocketService>,
    _producer: Box<dyn Bridge<EventBus>>,
}

impl Component for Main {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            wss: None,
            _producer: EventBus::bridge(ctx.link().callback(Msg::HandleMsg)),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Connect => {
                if self.wss.is_some() {
                    log::error!("Already connected!");
                }
                self.wss = Some(WebsocketService::new());
                true
            }
            Msg::Disconnect => {
                if self.wss.is_none() {
                    log::error!("Not connected!");
                }
                self.wss = None;
                true
            }
            Msg::Start => match &self.wss {
                None => {
                    log::error!("Not connected!");
                    true
                }
                Some(wss) => {
                    wss.start();
                    true
                }
            },
            Msg::Stop => match &self.wss {
                None => {
                    log::error!("Not connected!");
                    true
                }
                Some(wss) => {
                    wss.stop();
                    true
                }
            },
            Msg::HandleMsg(s) => {
                match s.as_str() {
                    "ws_disconnect" => {
                        self.wss = None;
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
        html! {
            <div>
                <Connector connected={ self.wss.is_some() } parent_cb={ctx.link().callback(|message| message)} />
                <Controls connected={ self.wss.is_some() } parent_cb={ctx.link().callback(|message| message)} />
            </div>
        }
    }
}
