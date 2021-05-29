use dprint_core::{
    configuration::{ConfigKeyMap, GlobalConfiguration, ResolveConfigurationResult},
    plugins::{PluginHandler, PluginInfo},
    types::ErrBox,
};
use dprint_plugin_mesonbuild::*;
use std::path::Path;

const ORIGIN_FILE_PATH: &str = "data/meson.build";
const ORIGIN_CONTENT: &str = include_str!("data/meson.build");

fn format_with_host(_: &Path, _: String, _: &ConfigKeyMap) -> Result<String, ErrBox> {
    Ok("".to_string())
}

#[test]
fn format0() {
    const FORMAT_CONTENT: &str = include_str!("data/0/meson.build");
    let config: Configuration = toml::from_str(include_str!("data/0/config.toml")).unwrap();

    let result = MesonPluginHandler::default().format_text(
        &Path::new(ORIGIN_FILE_PATH),
        ORIGIN_CONTENT,
        &config,
        format_with_host,
    );
    let result = result.unwrap();

    assert_eq!(result, FORMAT_CONTENT);
}

#[test]
fn format1() {
    const FORMAT_CONTENT: &str = include_str!("data/1/meson.build");
    let config: Configuration = toml::from_str(include_str!("data/1/config.toml")).unwrap();

    let result = MesonPluginHandler::default().format_text(
        &Path::new(ORIGIN_FILE_PATH),
        ORIGIN_CONTENT,
        &config,
        format_with_host,
    );
    let result = result.unwrap();

    assert_eq!(result, FORMAT_CONTENT);
}
