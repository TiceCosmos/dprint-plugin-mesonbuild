use dprint_core::{
    configuration::ConfigKeyMap,
    plugins::{FormatResult, SyncPluginHandler},
};
use dprint_plugin_mesonbuild::*;
use std::path::Path;

const ORIGIN_CONTENT: &str = include_str!("data/meson.build");
const CONFIG_CONTENT: &str = include_str!("data/config.toml");

fn format_with_host(_: &Path, _: String, _: &ConfigKeyMap) -> FormatResult {
    Ok(Some("".to_string()))
}

fn assert_with_config(expected: &str, toml_config: &str) {
    let config = toml::from_str(toml_config).unwrap();

    let result = MesonPluginHandler::default()
        .format(
            &Path::new("meson.build"),
            ORIGIN_CONTENT,
            &config,
            format_with_host,
        )
        .unwrap()
        .unwrap();

    for (a, b) in result.lines().zip(expected.lines()) {
        assert_eq!(a, b);
    }

    assert_eq!(result.lines().count(), expected.lines().count());
}

#[test]
fn format_file_0() {
    assert_with_config(ORIGIN_CONTENT, CONFIG_CONTENT);
}

#[test]
fn format_file_1() {
    assert_with_config(
        include_str!("data/1/meson.build"),
        include_str!("data/1/config.toml"),
    );
}

#[test]
fn format_file_2() {
    assert_with_config(
        include_str!("data/2/meson.build"),
        include_str!("data/2/config.toml"),
    );
}
