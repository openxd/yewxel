use yew::{html, Component, Properties};

#[derive(PartialEq, Properties)]
pub struct XMessageProps {}

pub struct XMessage;

impl Component for XMessage {
    type Message = ();
    type Properties = XMessageProps;
    fn create(_ctx: &yew::Context<Self>) -> Self {
        XMessage
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> yew::Html {
        return html! {
            <div></div>
        };
    }
}
