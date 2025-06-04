use std::env;

fn main() {
    // Special hack for jetbrains IDE like RustRover.
    if let Ok(rustc_wrapper) = std::env::var("RUSTC_WRAPPER") {
        if rustc_wrapper.to_lowercase().contains("jetbrains") || rustc_wrapper.to_lowercase().contains("jetbrains") {
            println!("cargo:warning=Detected RustRover IDE environment, skipping strict feature checks");
            return;
        }
    }

    let exclusive_features = [
        ("raspberry", "CARGO_FEATURE_RASPBERRY"),
        ("jetson", "CARGO_FEATURE_JETSON"),
        ("mock", "CARGO_FEATURE_MOCK"),
    ];

    let enabled: Vec<&str> = exclusive_features
        .iter()
        .filter(|(_, var)| env::var(var).is_ok())
        .map(|(name, _)| *name)
        .collect();

    match enabled.len() {
        0 => panic!(
            "You must enable exactly one of the following features: {}",
            exclusive_features.iter().map(|(name, _)| *name).collect::<Vec<_>>().join(", ")
        ),
        1 => {} // Ok
        _ => panic!("Features {:?} are mutually exclusive - enable only one.", enabled),
    }
}
