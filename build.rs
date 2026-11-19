use std::path::PathBuf;

fn main() {
    let ui = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap()).join("ui");
    // Dependents' build scripts read this as `DEP_NYANPASU_SLINT_PROJECT` and
    // map a library path to it, so the library keeps its own edition.
    println!(
        "cargo::metadata=project={}",
        ui.join("slint-project.json").display()
    );

    // The render tests' windows, which import the library as applications do.
    #[cfg(feature = "testing")]
    {
        let config = slint_build::CompilerConfiguration::new()
            .with_style("fluent".into())
            .with_edition((1, 19))
            .with_library_paths([("nyanpasu-slint".into(), ui.join("slint-project.json"))].into());
        slint_build::compile_with_config("tests/ui/harness.slint", config)
            .expect("the test harness compiles");
    }
}
