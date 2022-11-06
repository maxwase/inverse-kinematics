use inverse_kinematics::KinematicsApp;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let app = KinematicsApp::default();

    let native_options = eframe::NativeOptions {
        maximized: true,
        ..Default::default()
    };
    eframe::run_native(
        "Inverse kinematics",
        native_options,
        Box::new(|_| Box::new(app)),
    );
}

// when compiling to web using trunk.
#[cfg(target_arch = "wasm32")]
fn main() {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    let web_options = eframe::WebOptions::default();
    eframe::start_web(
        "inverse_kinematics",
        web_options,
        Box::new(|_| Box::new(KinematicsApp::default())),
    )
    .expect("failed to start eframe");
}
