pub use log::debug;
pub use log::error;
pub use log::info;
pub use log::log;
pub use log::trace;
pub use log::warn;

pub fn init_logger() {
    let _ = env_logger::builder()
        .format_timestamp(None)
        .format_source_path(true)
        .format_module_path(false)
        .format_target(false)
        .format_level(false)
        .try_init();
}
