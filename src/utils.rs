pub(crate) fn console_log(message: String) {
    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&message));
}

// Animations
#[cfg(all(feature = "element-x-tooltip", feature = "element-x-button"))]
pub(crate) fn new_animation(
    target: &web_sys::Element,
    keyframes: &js_sys::Object,
    duration: f64,
    easing: &crate::CSSEasing,
) -> web_sys::Animation {
    use web_sys::{AnimationEffect, KeyframeEffect, OptionalEffectTiming};
    let keyframe =
        KeyframeEffect::new_with_opt_element_and_keyframes(Some(&target), Some(keyframes)).unwrap();
    let animation_effect = AnimationEffect::from(keyframe);

    let mut timing = OptionalEffectTiming::new();
    timing.duration(&wasm_bindgen::JsValue::from_f64(duration));
    timing.easing(easing.to_string().as_str());
    animation_effect.update_timing_with_timing(&timing).unwrap();

    let animation = web_sys::Animation::new_with_effect(Some(&animation_effect)).unwrap();
    animation
}

#[cfg(feature = "feature-intl")]
pub(crate) async fn load_text_content(url: web_sys::Url) -> Result<String, wasm_bindgen::JsValue> {
    use wasm_bindgen::JsCast;
    use wasm_bindgen_futures::JsFuture;
    use web_sys::{window, Request, RequestInit, RequestMode, Response};

    let mut req_opts = RequestInit::new();
    req_opts.method("GET");
    req_opts.mode(RequestMode::Cors);

    let req = Request::new_with_str_and_init(&String::from(url.to_string()), &req_opts)?;
    let window = window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&req)).await?;

    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();
    let text = JsFuture::from(resp.text()?).await?;

    Ok(text.as_string().unwrap())
}
