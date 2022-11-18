use std::collections::HashMap;

use fluent::{FluentArgs, FluentBundle, FluentError, FluentResource};
use wasm_bindgen::JsValue;

pub struct Intl {
    intl_bundle: FluentBundle<FluentResource>,
    urls_loaded: Vec<web_sys::Url>,
}

#[derive(Debug)]
pub enum IntlError {
    JS(JsValue),
    Fluent,
}

impl From<JsValue> for IntlError {
    fn from(v: JsValue) -> Self {
        Self::JS(v)
    }
}

impl Intl {
    pub fn new(locale: String) -> Self {
        Intl {
            intl_bundle: FluentBundle::new(vec![locale
                .parse()
                .expect("Provided locale id is incorrect")]),
            urls_loaded: vec![],
        }
    }

    pub fn is_loaded(&self) -> bool {
        self.urls_loaded.len() > 0
    }

    pub fn load(&mut self, url: web_sys::Url, content: String) -> Result<(), IntlError> {
        let resource = FluentResource::try_new(content).map_err(|_| IntlError::Fluent)?;
        self.intl_bundle
            .add_resource(resource)
            .map_err(|_e| IntlError::Fluent)?;
        self.urls_loaded.push(url);
        Ok(())
    }

    pub fn change(&mut self, locale: String) {
        self.intl_bundle = FluentBundle::new(vec![locale
            .parse()
            .expect("Provided locale id is incorrect")]);
        self.urls_loaded = vec![];
    }

    pub fn get(
        &self,
        id: &str,
        args: HashMap<String, String>,
    ) -> Result<Option<String>, Vec<FluentError>> {
        let pattern = self.intl_bundle.get_message(id).map(|ok| ok.value());
        if let Some(Some(pattern)) = pattern {
            let mut errors = vec![];
            let fluent_args =
                FluentArgs::from_iter(args.iter().map(|(k, v)| (k.clone(), v.clone())));
            let message = self
                .intl_bundle
                .format_pattern(pattern, Some(&fluent_args), &mut errors);

            if errors.len() > 0 {
                return Err(errors);
            }

            Ok(Some(message.to_string()))
        } else {
            Ok(None)
        }
    }
}

impl PartialEq for Intl {
    fn eq(&self, other: &Self) -> bool {
        self.intl_bundle.locales.eq(&other.intl_bundle.locales)
            && self.urls_loaded.eq(&other.urls_loaded)
    }
}
