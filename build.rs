use core::panic;
use cssparser::ToCss;
use lightningcss::rules::CssRule;
use lightningcss::selector::Selectors;
use lightningcss::stylesheet::{ParserOptions, PrinterOptions, StyleSheet};
use parcel_selectors::attr::AttrSelectorOperator;
use parcel_selectors::parser::{Combinator, Component, Selector};
use parcel_selectors::SelectorList;
use std::fmt::Write;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Read, Write as IOWrite};
use std::{
    env::{var, vars},
    path::Path,
};

const PRINTER_OPTIONS: PrinterOptions = PrinterOptions {
    minify: true,
    source_map: None,
    targets: None,
    analyze_dependencies: None,
    pseudo_classes: None,
};

fn main() {

    if std::env::var("DOCS_RS").is_ok() {
        return;
    }

    let mut elements = vec![];
    let mut ui = None;

    for (name, val) in vars() {
        if name.starts_with("CARGO_FEATURE_ELEMENT_") && &val == "1" {
            elements.push(String::from(&name[22..]).to_lowercase().replace("_", "-"));
        } else if name.starts_with("CARGO_FEATURE_THEME_") && &val == "1" && ui.is_none() {
            ui = Some(String::from(&name[20..]).to_lowercase());
        }
    }

    let dark_mode = var("CARGO_FEATURE_MODE_DARK").unwrap_or(String::from("0")) == "1";
    let light_mode = var("CARGO_FEATURE_MODE_LIGHT").unwrap_or(String::from("0")) == "1";

    if ui.is_none() {
        panic!("Not provided a UI feature.");
    }

    if !light_mode && !dark_mode {
        panic!("Atleast one mode required.");
    }

    let mut base_stylesheet: String = String::new();

    let out_dir = var("OUT_DIR").unwrap();

    construct_stylesheet(&mut base_stylesheet, "styles/shadow.css", &elements);
    construct_stylesheet(&mut base_stylesheet, "styles/base.css", &elements);

    if dark_mode && light_mode {
        write_to_out(&out_dir, "base.css", base_stylesheet);

        let mut light_stylesheet = String::new();
        construct_stylesheet(
            &mut light_stylesheet,
            format!("styles/{}.css", ui.clone().unwrap()),
            &elements,
        );
        write_to_out(&out_dir, "light.css", light_stylesheet);

        let mut dark_stylesheet = String::new();
        construct_stylesheet(
            &mut dark_stylesheet,
            format!("styles/{}-dark.css", ui.clone().unwrap()),
            &elements,
        );
        write_to_out(&out_dir, "dark.css", dark_stylesheet);
    } else if dark_mode {
        construct_stylesheet(
            &mut base_stylesheet,
            format!("styles/{}-dark.css", ui.clone().unwrap()),
            &elements,
        );
        write_to_out(&out_dir, "base.css", base_stylesheet);
    } else {
        construct_stylesheet(
            &mut base_stylesheet,
            format!("styles/{}.css", ui.clone().unwrap()),
            &elements,
        );
        write_to_out(&out_dir, "base.css", base_stylesheet);
    }
}

fn write_to_out<P: AsRef<Path>>(out_dir: P, file_name: &'static str, contents: String) {
    let file_path = out_dir.as_ref();
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .append(false)
        .create(true)
        .open(file_path.join(file_name))
        .unwrap();

    let mut file_writer = BufWriter::new(file);
    file_writer.write_all(contents.as_bytes()).unwrap();
}

fn construct_stylesheet<P: AsRef<Path>>(
    rewrite_str: &mut String,
    file_name: P,
    elements: &Vec<String>,
) {
    let mut file = File::open(file_name).unwrap();
    let mut css_str = String::new();
    file.read_to_string(&mut css_str).unwrap();
    let stylesheet = StyleSheet::parse(&css_str, ParserOptions::default()).unwrap();
    rewrite_rule_list(rewrite_str, &stylesheet.rules.0, elements);
}

fn rewrite_rule_list<'i>(
    rewrite_str: &mut String,
    rule_list: &Vec<CssRule>,
    elements: &Vec<String>,
) {
    use lightningcss::traits::ToCss as LightningToCss;
    for rule in rule_list.iter().rev() {
        match rule {
            CssRule::Style(style_rule) => {
                let selectors_str_opt = rewrite_selectors(&style_rule.selectors, &elements);
                if let Some(selectors_str) = selectors_str_opt {
                    let definition =
                        LightningToCss::to_css_string(&style_rule.declarations, PRINTER_OPTIONS)
                            .unwrap();
                    write!(rewrite_str, "{} {{{}}}", selectors_str, definition).unwrap();
                }
            }
            CssRule::Media(media) => {
                write!(
                    rewrite_str,
                    "@media {} {{ ",
                    media.query.to_css_string(PRINTER_OPTIONS).unwrap()
                )
                .unwrap();
                rewrite_rule_list(rewrite_str, &media.rules.0, elements);
                write!(rewrite_str, "}} ").unwrap()
            }
            _ => {}
        }
    }
}

fn rewrite_selectors(
    selectors: &SelectorList<Selectors>,
    elements: &Vec<String>,
) -> Option<String> {
    let mut selectors_str = String::new();
    let mut first = true;
    let mut generated = false;
    for selector in selectors.0.iter() {
        let selector_str_opt = rewrite_selector(selector, &elements);
        if let Some(selector_str) = selector_str_opt {
            if !first {
                write!(&mut selectors_str, ",").unwrap();
            }
            generated = true;
            write!(&mut selectors_str, "{}", selector_str).unwrap();
            first = false;
        }
    }

    if !generated {
        None
    } else {
        Some(selectors_str)
    }
}

fn rewrite_selector(selector: &Selector<Selectors>, elements: &Vec<String>) -> Option<String> {
    let mut component_strs = vec![];
    let mut first = true;
    let mut selector_iter = selector.iter();
    let mut combinator: Option<Combinator> = None;
    let mut part_str: Option<String> = None;
    let mut x_element = None;

    while first || combinator.is_some() {
        if let Some(combinator_cloned) = combinator.clone() {
            component_strs.push(combinator_cloned.to_css_string());
        }
        let mut component_str = String::new();

        for component in &mut selector_iter {
            match component {
                Component::LocalName(local_name) => {
                    let css_name = local_name.to_css_string();
                    if css_name.starts_with("x-") && !elements.contains(&css_name) {
                        return None;
                    } else if css_name.starts_with("x-") {
                        x_element = Some(css_name.clone());
                        write!(&mut component_str, ".{}", css_name).unwrap();
                    } else {
                        write!(&mut component_str, "{}", css_name).unwrap();
                    }
                }
                Component::AttributeInNoNamespace {
                    local_name,
                    operator,
                    value,
                    case_sensitivity: _,
                    never_matches: _,
                } => {
                    if let AttrSelectorOperator::Equal = operator {
                        write!(
                            &mut component_str,
                            ".{}-{}",
                            local_name.to_css_string(),
                            value.to_css_string()
                        )
                        .unwrap();
                    } else {
                        write!(&mut component_str, "{}", component.to_css_string()).unwrap()
                    }
                }
                Component::Part(idents) => {
                    part_str = Some(idents.as_ref()[0].to_css_string());
                }
                Component::AttributeInNoNamespaceExists {
                    local_name,
                    local_name_lower: _,
                } => {
                    write!(&mut component_str, ".{}", local_name.to_css_string()).unwrap();
                }
                Component::Negation(selectors) => {
                    let mut selector_strs = vec![];
                    for selector in selectors.as_ref() {
                        if let Some(selector_str) = rewrite_selector(selector, elements) {
                            selector_strs.push(selector_str);
                        }
                    }
                    if selector_strs.len() > 0 {
                        write!(&mut component_str, ":not({})", selector_strs.join(", ")).unwrap();
                    }
                }
                Component::PseudoElement(pseudo) => {
                    write!(
                        &mut component_str,
                        "{}",
                        lightningcss::traits::ToCss::to_css_string(&pseudo, PRINTER_OPTIONS)
                            .unwrap()
                    )
                    .unwrap();
                }
                Component::NonTSPseudoClass(class) => {
                    write!(
                        &mut component_str,
                        "{}",
                        lightningcss::traits::ToCss::to_css_string(&class, PRINTER_OPTIONS)
                            .unwrap()
                    )
                    .unwrap();
                }
                _ => {
                    write!(&mut component_str, "{}", component.to_css_string()).unwrap();
                }
            }
        }

        if let Some(part_str_ok) = part_str.clone() {
            if let Some(x_element_ok) = x_element.clone() {
                write!(&mut component_str, " .{}-{}", x_element_ok, part_str_ok).unwrap();

                part_str = None;
            }
        }
        if component_str.len() > 0 {
            component_strs.push(component_str);
        }
        combinator = selector_iter.next_sequence();
        first = false;
    }

    component_strs.reverse();
    Some(component_strs.join(""))
}
