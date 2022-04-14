pub fn init_logging() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Info).expect("Couldn't initialize logger!");
        } else {
            env_logger::builder()
                .format_target(true)
                .format_timestamp_micros()
                .filter_level(log::LevelFilter::Info)
                .init();
        }
    }
}
