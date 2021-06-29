use dprint_core::configuration::get_unknown_property_diagnostics;
use dprint_core::configuration::{
    ConfigKeyMap, ConfigurationDiagnostic, GlobalConfiguration, ResolveConfigurationResult,
};
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
    /// indent width
    pub indent_width: u8,
    /// align at `:`
    pub align_colon: bool,
    /// spaces before `:`
    pub space_before_colon: bool,
    /// spaces before `( [ {` and after `) ] }`
    pub space_inner_bracket: bool,
    /// wrap before `) ] }`
    pub wrap_close_brace: bool,
    /// nowrap before name argument
    pub nowrap_before_name: bool,
}
pub const DEFAULT_CONFIGURATION: Configuration = Configuration {
    indent_width: 2,
    align_colon: false,
    space_before_colon: false,
    space_inner_bracket: false,
    wrap_close_brace: true,
    nowrap_before_name: true,
};
impl Default for Configuration {
    fn default() -> Self {
        DEFAULT_CONFIGURATION
    }
}

pub fn resolve_config(
    config: ConfigKeyMap,
    global_config: &GlobalConfiguration,
) -> ResolveConfigurationResult<Configuration> {
    let mut builder = ConfigurationBuilder::new(config);

    let mut config = Configuration::default();
    if let Some(value) = global_config.indent_width {
        config.indent_width = value;
    }
    builder.get_nullable_value(&mut config.indent_width, "indentWidth");
    builder.get_nullable_value(&mut config.align_colon, "alignColon");
    builder.get_nullable_value(&mut config.space_before_colon, "spaceBeforeColon");
    builder.get_nullable_value(&mut config.space_inner_bracket, "spaceInnerBracket");
    builder.get_nullable_value(&mut config.wrap_close_brace, "wrapCloseBrace");
    builder.get_nullable_value(&mut config.nowrap_before_name, "nowrapBeforeName");

    ResolveConfigurationResult {
        config,
        diagnostics: builder.extend(),
    }
}

struct ConfigurationBuilder {
    config: ConfigKeyMap,
    diagnostics: Vec<ConfigurationDiagnostic>,
}
impl ConfigurationBuilder {
    fn new(config: ConfigKeyMap) -> Self {
        Self {
            config,
            diagnostics: Vec::new(),
        }
    }
    fn extend(self) -> Vec<ConfigurationDiagnostic> {
        let mut data = self;
        data.diagnostics
            .extend(get_unknown_property_diagnostics(data.config));
        data.diagnostics
    }
    fn get_nullable_value<T>(&mut self, store: &mut T, key: &'static str)
    where
        T: FromStr,
        <T as FromStr>::Err: fmt::Display,
    {
        if let Some(value) = dprint_core::configuration::get_nullable_value(
            &mut self.config,
            key,
            &mut self.diagnostics,
        ) {
            *store = value;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dprint_core::configuration::{resolve_global_config, ConfigKeyValue};

    #[test]
    fn resolve_null_config() {
        let global_config = resolve_global_config(ConfigKeyMap::new()).config;
        assert_eq!(
            resolve_config(ConfigKeyMap::new(), &global_config).config,
            DEFAULT_CONFIGURATION
        );
    }

    #[test]
    fn resolve_full_config() {
        let global_config = resolve_global_config(ConfigKeyMap::new()).config;

        let changed_config = Configuration {
            indent_width: DEFAULT_CONFIGURATION.indent_width * 2,
            align_colon: !DEFAULT_CONFIGURATION.align_colon,
            space_before_colon: !DEFAULT_CONFIGURATION.space_before_colon,
            space_inner_bracket: !DEFAULT_CONFIGURATION.space_inner_bracket,
            wrap_close_brace: !DEFAULT_CONFIGURATION.wrap_close_brace,
            nowrap_before_name: !DEFAULT_CONFIGURATION.wrap_close_brace,
        };

        let key_map = vec![
            (
                "indentWidth",
                ConfigKeyValue::Number(changed_config.indent_width as i32),
            ),
            (
                "alignColon",
                ConfigKeyValue::Bool(changed_config.align_colon),
            ),
            (
                "spaceBeforeColon",
                ConfigKeyValue::Bool(changed_config.space_before_colon),
            ),
            (
                "spaceInnerBracket",
                ConfigKeyValue::Bool(changed_config.space_inner_bracket),
            ),
            (
                "wrapCloseBrace",
                ConfigKeyValue::Bool(changed_config.wrap_close_brace),
            ),
            (
                "nowrapBeforeName",
                ConfigKeyValue::Bool(changed_config.nowrap_before_name),
            ),
        ]
        .into_iter()
        .map(|(k, v)| (k.to_string(), v))
        .collect::<ConfigKeyMap>();

        assert_eq!(
            resolve_config(key_map, &global_config).config,
            changed_config
        );
    }
}
