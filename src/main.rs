mod app;

use app::App;
use log::info;
use wasm_bindgen::JsValue;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    let object = JsValue::from("world");
    info!("Hello {}", object.as_string().unwrap());

    yew::Renderer::<App>::new().render();
}
