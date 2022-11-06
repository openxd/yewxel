use yew::{Children, Component, html, Properties};

#[derive(Properties, PartialEq)]
pub struct XLabelProps {
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub hidden: bool,
    #[prop_or_default]
    pub children: Children,
}

pub struct XLabel;

impl Component for XLabel {
    type Message = ();

    type Properties = XLabelProps;

    fn create(ctx: &yew::Context<Self>) -> Self {
        XLabel
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let mut classes = String::from("x-label");
        let prop = ctx.props();

        if prop.disabled {
            classes.push_str(" disabled");
        }

        if prop.hidden {
            classes.push_str(" hidden");
        }
        
        html! {
            <div class={classes}>
                <div class="x-label-contents">
                    {for ctx.props().children.iter()}
                </div>
            </div>
        }
    }
}
