fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();

    if target_os == "linux" {
        #[cfg(target_os = "linux")]
        {
            if pkg_config::probe_library("libpipewire-0.3").is_err() {
                eprintln!("Missing libpipewire-0.3");
            }
        }
    }

    tauri_build::build();
}
