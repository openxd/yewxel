//! `XLabel` is a generic component similiar to `<label>`, but can be used in any place rather than
//! just inside HTML forms.
//!
//! ```
//! <XButton>
//!     <XLabel>{"My Button"}</XLabel>
//! </XButton>
//! ```

use yew::{Children, Component, html, Properties};

/// Properties for XLabel element
#[derive(Properties, PartialEq)]
pub struct XLabelProps {
    /// Whether that label is disabled or not
    #[prop_or_default]
    pub disabled: bool,
    /// To hide the label
    #[prop_or_default]
    pub hidden: bool,
    /// Inner contents of the label
    #[prop_or_default]
    pub children: Children,
    /// Classes to apply for root element
    #[prop_or_default]
    pub class: Option<String>,
    /// Styles to apply for root element
    #[prop_or_default]
    pub style: Option<String>,
}

/// XLabel component
pub struct XLabel;

impl Component for XLabel {
    type Message = ();

    type Properties = XLabelProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        XLabel
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let mut classes = String::from("x-label");
        let prop = ctx.props();

        if let Some(user_class) = prop.class.clone() {
            classes.push_str(user_class.trim());
        }

        if prop.disabled {
            classes.push_str(" disabled");
        }

        if prop.hidden {
            classes.push_str(" hidden");
        }
        
        html! {
            <div style={prop.style.clone().unwrap_or(String::from(""))} class={classes}>
                <div class="x-label-contents">
                    {for ctx.props().children.iter()}
                </div>
            </div>
        }
    }
}
