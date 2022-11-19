use std::{cell::RefCell, rc::Rc};

use fluent::{FluentArgs, FluentBundle, FluentError, FluentResource};
use wasm_bindgen::JsValue;

#[derive(Clone)]
pub struct Intl {
    intl_bundle: Rc<RefCell<FluentBundle<FluentResource>>>,
    urls_loaded: Vec<web_sys::Url>,
    locale: String,
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
            locale: locale.clone(),
            intl_bundle: Rc::new(RefCell::new(FluentBundle::new(vec![locale
                .parse()
                .expect("Provided locale id is incorrect")]))),
            urls_loaded: vec![],
        }
    }

    pub fn is_loaded(&self) -> bool {
        self.urls_loaded.len() > 0
    }

    pub fn load(&mut self, url: web_sys::Url, content: String) -> Result<(), IntlError> {
        let resource = FluentResource::try_new(content).map_err(|_| IntlError::Fluent)?;
        self.intl_bundle
            .borrow_mut()
            .add_resource(resource)
            .map_err(|_e| IntlError::Fluent)?;
        self.urls_loaded.push(url);
        Ok(())
    }

    pub fn change(&mut self, locale: String) {
        self.intl_bundle = Rc::new(RefCell::new(FluentBundle::new(vec![locale
            .parse()
            .expect("Provided locale id is incorrect")])));
        self.urls_loaded = vec![];
    }

    pub fn locale(&self) -> Option<String> {
        self.intl_bundle
            .borrow()
            .locales
            .first()
            .map(|l| l.to_string())
    }

    pub fn get<'a>(
        &self,
        id: &'a str,
        args: Option<&FluentArgs<'a>>,
    ) -> Result<Option<String>, Vec<FluentError>> {
        let intl_bundle = self.intl_bundle.borrow();
        let pattern = intl_bundle.get_message(id).map(|ok| ok.value());
        if let Some(Some(pattern)) = pattern {
            let mut errors = vec![];
            let message = intl_bundle.format_pattern(pattern, args, &mut errors);

            if errors.len() > 0 {
                return Err(errors);
            }

            Ok(Some(message.to_string()))
        } else {
            Ok(None)
        }
    }

    pub fn get_attribute<'a>(
        &self,
        id: &'a str,
        attribute: &'a str,
        args: Option<&FluentArgs<'a>>,
    ) -> Result<Option<String>, Vec<FluentError>> {
        let intl_bundle = self.intl_bundle.borrow();
        let message = intl_bundle.get_message(id);

        if let Some(message) = message {
            if let Some(attr) = message.get_attribute(attribute) {
                let mut errors = vec![];
                let message = intl_bundle.format_pattern(attr.value(), args, &mut errors);

                if errors.len() > 0 {
                    return Err(errors);
                }

                Ok(Some(message.to_string()))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
}

impl PartialEq for Intl {
    fn eq(&self, other: &Self) -> bool {
        self.locale.eq(&other.locale) && self.urls_loaded.eq(&other.urls_loaded)
    }
}
