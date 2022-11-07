#![recursion_limit = "1024"]
use yew::prelude::*;
use yewxel::{
    xbutton::XButton,
    xcontainer::{Mode, XContainer},
    xlabel::XLabel,
    xtooltip::XTooltip,
    ComputedSize,
};

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
                <XContainer size={ComputedSize::Small} mode={Mode::Auto}>
                    <XButton>
                        <XLabel>
                            {"Test"}
                        </XLabel>
                        <XTooltip>
                            <XLabel>
                                {"Test2"}
                            </XLabel>
                        </XTooltip>
                    </XButton>
                </XContainer>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<ExampleApp>();
}
