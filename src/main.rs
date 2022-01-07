#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let app = inverse_kinematics::KinematicsApp::default();

    let native_options = eframe::NativeOptions {
        maximized: true,
        ..Default::default()
    };
    eframe::run_native(Box::new(app), native_options);
}
