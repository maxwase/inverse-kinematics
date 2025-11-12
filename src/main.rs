use inverse_kinematics::KinematicsApp;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "Inverse kinematics",
        Default::default(),
        Box::new(|_| Ok(Box::<KinematicsApp>::default())),
    )
}

// when compiling to web using trunk.
#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast as _;
    use eframe::web_sys;

    const CANVAS_ID: &str = "inverse_kinematics";

    let web_options = eframe::WebOptions::default();
    let document = web_sys::window()
        .expect("No window")
        .document()
        .expect("No document");

    let canvas = document
        .get_element_by_id(CANVAS_ID)
        .expect("Failed to find the_canvas_id")
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .expect("the_canvas_id was not a HtmlCanvasElement");

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|_| Ok(Box::<KinematicsApp>::default())),
            )
            .await
            .expect("start kinematics app");
    });
}
