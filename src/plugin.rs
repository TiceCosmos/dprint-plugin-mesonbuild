use crate::configuration::Configuration;
use dprint_core::{
    configuration::{ConfigKeyMap, GlobalConfiguration, ResolveConfigurationResult},
    plugins::{FormatResult, PluginInfo, SyncPluginHandler},
};
use std::path::Path;

#[derive(Default)]
pub struct MesonPluginHandler {}

impl SyncPluginHandler<Configuration> for MesonPluginHandler {
    fn resolve_config(
        &mut self,
        config: ConfigKeyMap,
        global_config: &GlobalConfiguration,
    ) -> ResolveConfigurationResult<Configuration> {
        crate::configuration::resolve_config(config, global_config)
    }

    fn plugin_info(&mut self) -> PluginInfo {
        PluginInfo {
            name: env!("CARGO_PKG_NAME").to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            config_key: "mesonbuild".to_string(),
            file_extensions: vec![],
            file_names: vec!["meson.build", "meson_options.txt"]
                .into_iter()
                .map(|s| s.to_string())
                .collect(),
            help_url: "https://github.com/TiceCosmos/dprint-plugin-mesonbuild".to_string(),
            config_schema_url: "".to_string(),
            update_url: None,
        }
    }

    fn license_text(&mut self) -> String {
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/LICENSE")).into()
    }

    fn format(
        &mut self,
        _file_path: &Path,
        file_text: &str,
        config: &Configuration,
        mut _format_with_host: impl FnMut(&Path, String, &ConfigKeyMap) -> FormatResult,
    ) -> FormatResult {
        Ok(crate::format_text::format_text(file_text, config)?)
    }
}

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
dprint_core::generate_plugin_code!(MesonPluginHandler, MesonPluginHandler {});
