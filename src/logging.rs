use log::LevelFilter;

pub fn init() {
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(LevelFilter::Debug)
            .with_tag("RustdroidModding"),
    );
}

pub fn log_info(msg: &str) {
    log::info!("{}", msg);
}

pub fn log_error(msg: &str) {
    log::error!("{}", msg);
}

pub fn log_debug(msg: &str) {
    log::debug!("{}", msg);
}
