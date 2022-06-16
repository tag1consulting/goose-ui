use crate::components::main::Msg as MainMsg;
use yew::prelude::*;

pub enum Msg {
    Toggle,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub parent_cb: Callback<MainMsg>,
    pub connected: bool,
}

pub struct Connector {
    host: &'static str,
}

impl Component for Connector {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            host: "ws://127.0.0.1:5117",
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Toggle => {
                if ctx.props().connected {
                    ctx.props().parent_cb.emit(MainMsg::Disconnect);
                } else {
                    ctx.props().parent_cb.emit(MainMsg::Connect);
                }
                true
            }
            _ => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <div>
                <div>
                    <label for="host">{ "Hostname: " }</label>
                    <input disabled={ ctx.props().connected } name="host" type="text" value={ self.host } />
                </div>
                <div>
                    <button disabled={ ctx.props().connected } onclick={link.callback(|_| Msg::Toggle)}>{ if ctx.props().connected { "Connected!"  } else { "Connect" } }</button>
                </div>
            </div>
        }
    }
}
