//! `XMessage` is used to display a specific localized text from FTL locale file(s). You must
//! enable `"feature-intl"` and `"element-x-message"` features to use this.
//!
//! # Usage
//!
//! ```
//! <XContainer ftls={vec![Url::new("http://127.0.0.1:8000/path/to/en-US.ftl")]} locale="en-US">
//!     <XMessage href="#confirm" args="name:My Document.docx,created:1h"/>
//! </XCotainer>
//! ```
//!
use fluent::{FluentArgs, FluentValue};
use regex::Regex;
use titlecase::titlecase;
use yew::{context::ContextHandle, Callback, Component, Properties};

use crate::intl::Intl;

#[doc(hidden)]
pub enum XMessageMessage {
    IntlLoaded(Intl),
}

/// Props to XMessage component
#[derive(PartialEq, Properties)]
pub struct XMessageProps {
    /// Link to the translation. Syntaxes:- `"#hello-world","#confirm.yes"`
    pub href: String,
    /// Arguments. Syntax:- `"id-one:Id one text,id-two: Id\\, two,id-three:4"`
    #[prop_or_default]
    pub args: Option<String>,
    /// Turn the text into titlecase if the locale is English
    #[prop_or_default]
    pub autocapitalize: bool,
    /// Whether to show an ellipsis at the end of the message
    #[prop_or_default]
    pub ellipsis: bool,
    /// On the locale change
    #[prop_or_default]
    pub onchange: Option<Callback<XMessageEvent>>,
}

/// XMessage component
pub struct XMessage {
    intl_state: Intl,
    text: Option<String>,
    _intl_listener: ContextHandle<Intl>,
}

/// Events occurring from XMessage component
pub struct XMessageEvent {
    locale: Option<String>,
    text: Option<String>,
}

impl XMessageEvent {
    /// Current locale as an identifier. `None` if no locale set. Example:- `"en-US"`
    pub fn locale(&self) -> Option<String> {
        self.locale.clone()
    }

    /// Text contents if available an entry in loaded FTL files.
    pub fn text(&self) -> Option<String> {
        self.text.clone()
    }
}

impl XMessage {
    fn get_text(&self, ctx: &yew::Context<Self>) -> Option<String> {
        let (id, attr) = parse_href(ctx.props().href.clone());
        let args = ctx.props().args.as_ref().map(|a| parse_args(a));

        if let Some(attr) = attr {
            self.intl_state
                .get_attribute(&id, &attr, args.as_ref())
                .unwrap()
        } else {
            self.intl_state.get(&id, args.as_ref()).unwrap()
        }
    }
}

impl Component for XMessage {
    type Message = XMessageMessage;
    type Properties = XMessageProps;
    fn create(ctx: &yew::Context<Self>) -> Self {
        let (intl_state, _intl_listener) = ctx
            .link()
            .context::<Intl>(ctx.link().callback(XMessageMessage::IntlLoaded))
            .expect("Intl feature not enabled");

        XMessage {
            text: None,
            intl_state,
            _intl_listener,
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            XMessageMessage::IntlLoaded(intl_state) => {
                self.intl_state = intl_state.clone();

                self.text = self.get_text(ctx);

                if let Some(onchange) = ctx.props().onchange.clone() {
                    onchange.emit(XMessageEvent {
                        locale: self.intl_state.locale(),
                        text: self.text.clone(),
                    });
                }
                true
            }
        }
    }

    fn changed(&mut self, ctx: &yew::Context<Self>, old_props: &Self::Properties) -> bool {
        if ctx.props().href != old_props.href || ctx.props().args != old_props.args {
            self.text = self.get_text(ctx);
        }
        true
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let mut text = self.text.clone().unwrap_or(String::new());

        if ctx.props().autocapitalize {
            text = titlecase(&text);
        }

        if ctx.props().ellipsis {
            text.push_str("...");
        }

        return text.into();
    }
}

fn parse_href(href: String) -> (String, Option<String>) {
    let mut new_href = href.clone();
    if href.starts_with('#') {
        new_href.remove(0);
    }

    if href.contains('.') {
        let mut splitted = href.split('.').into_iter();
        let id = splitted.next().unwrap();
        let attr = splitted.next().map(|a| String::from(a));
        (String::from(id), attr)
    } else {
        (new_href, None)
    }
}

fn parse_args<'a>(args_str: &'a str) -> FluentArgs<'a> {
    if args_str.trim() == "" {
        return FluentArgs::new();
    }
    let mut new_args = FluentArgs::new();

    let reg = Regex::new("(?:[^\\\\]),").unwrap();

    for arg_strs in reg.split(&args_str).into_iter() {
        let mut arg_strs_split = arg_strs.splitn(2, ':').into_iter();
        let key = arg_strs_split.next().unwrap();
        let value = arg_strs_split.next();

        if let Some(value) = value {
            if let Ok(parsed) = value.parse::<f64>() {
                new_args.set(key, parsed);
            } else {
                new_args.set(key, value.replace("\\", ""));
            }
        } else {
            new_args.set(key, FluentValue::None);
        }
    }

    new_args
}
