use inverse_kinematics::KinematicsApp;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "Inverse kinematics",
        Default::default(),
        Box::new(|_| Box::<KinematicsApp>::default()),
    )
}

// when compiling to web using trunk.
#[cfg(target_arch = "wasm32")]
fn main() {
    const CANVAS_ID: &str = "inverse_kinematics";

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                CANVAS_ID,
                web_options,
                Box::new(|_| Box::<KinematicsApp>::default()),
            )
            .await
            .expect("start kinematics app");
    });
}
