pub fn init_logger() {
    let _ = env_logger::builder()
        .format_timestamp(None)
        .format_source_path(true)
        .format_module_path(false)
        .format_target(false)
        .format_level(false)
        .try_init();
}
