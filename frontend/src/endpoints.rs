use wasm_bindgen::JsCast;

pub fn websocket_url() -> String {
    let scope = js_sys::global().unchecked_into::<web_sys::WorkerGlobalScope>();
    if let Some(url) = websocket_url_from_origin(&scope.origin()) {
        return url;
    }

    let location = scope.location();
    websocket_url_from_origin(&location.origin())
        .or_else(|| websocket_url_from_origin(&location.href()))
        .expect("worker origin must be http or https")
}

fn websocket_url_from_origin(origin: &str) -> Option<String> {
    let origin = origin.strip_prefix("blob:").unwrap_or(origin);

    if let Some(host) = origin.strip_prefix("https://") {
        Some(format!("wss://{}/ws", host.split('/').next().unwrap_or(host)))
    } else if let Some(host) = origin.strip_prefix("http://") {
        Some(format!("ws://{}/ws", host.split('/').next().unwrap_or(host)))
    } else {
        None
    }
}
