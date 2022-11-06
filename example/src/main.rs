#![recursion_limit="1024"]
use yew::prelude::*;
use yewxel::{xbutton::XButton, xcontainer::{XContainer, Mode}};

struct ExampleApp;

impl Component for ExampleApp {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self 
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <XContainer mode={Mode::Auto}>
                    <XButton>
                        {"Test"}
                    </XButton>
                </XContainer>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<ExampleApp>();
}
