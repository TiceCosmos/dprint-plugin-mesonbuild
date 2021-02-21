use dprint_core::configuration::{
    get_unknown_property_diagnostics, get_value, ConfigKeyMap, GlobalConfiguration,
    ResolveConfigurationResult,
};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Configuration {
    pub line_width: u32,
    pub indent_string: String,
}

pub fn resolve_config(
    config: ConfigKeyMap,
    global_config: &GlobalConfiguration,
) -> ResolveConfigurationResult<Configuration> {
    let mut config = config;
    let mut diagnostics = Vec::new();

    let line_width = get_value(
        &mut config,
        "line_width",
        global_config.line_width.unwrap_or(120),
        &mut diagnostics,
    );

    let indent_string = get_value(
        &mut config,
        "indent_string",
        if global_config.use_tabs.unwrap_or(false) {
            "\t".to_string()
        } else {
            format!(
                "{:1$}",
                "",
                global_config.indent_width.unwrap_or(4) as usize
            )
        },
        &mut diagnostics,
    );

    diagnostics.extend(get_unknown_property_diagnostics(config));

    ResolveConfigurationResult {
        config: Configuration {
            line_width,
            indent_string,
        },
        diagnostics,
    }
}
