use super::configuration::{resolve_config, Configuration};
use dprint_core::generate_plugin_code;
use std::path::Path;

fn get_plugin_config_key() -> String {
    String::from("mesonbuild")
}

fn get_plugin_file_extensions() -> Vec<String> {
    vec![String::from("meson.build")]
}

fn get_plugin_help_url() -> String {
    String::new()
}

fn get_plugin_config_schema_url() -> String {
    String::new()
}

fn get_plugin_license_text() -> String {
    std::str::from_utf8(include_bytes!("../LICENSE"))
        .unwrap()
        .into()
}

fn format_text(
    _file_path: &Path,
    file_text: &str,
    config: &Configuration,
) -> Result<String, String> {
    crate::format_text::format_text(file_text, config)
}

generate_plugin_code!();
