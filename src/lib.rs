mod app;
mod segment;
pub use app::KinematicsApp;

#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::{self, prelude::*};

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn web_main(canvas_id: &str) -> Result<(), eframe::wasm_bindgen::JsValue> {
    let app = KinematicsApp::default();
    eframe::start_web(canvas_id, Box::new(|_| Box::new(app)))
}
