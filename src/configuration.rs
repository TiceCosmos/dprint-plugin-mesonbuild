use dprint_core::configuration::get_unknown_property_diagnostics;
use dprint_core::configuration::{
    ConfigKeyMap, ConfigurationDiagnostic, GlobalConfiguration, ResolveConfigurationResult,
};
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Configuration {
    /// indent width
    pub indent_width: u8,
    /// spaces before ":"
    pub space_before_colon: bool,
    /// spaces before "(", "[", "{" and after "}", "]", ")"
    pub space_inner_bracket: bool,
}
impl Default for Configuration {
    fn default() -> Self {
        Self {
            indent_width: 2,
            space_before_colon: false,
            space_inner_bracket: false,
        }
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

    builder.get_nullable_value(&mut config.indent_width, "indent_width");
    builder.get_nullable_value(&mut config.space_before_colon, "space_before_colon");
    builder.get_nullable_value(&mut config.space_inner_bracket, "space_before_colon");

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
