
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MCLConfig {
    pub general: GeneralConfig,
    pub network: NetworkConfig,
    pub java: JavaConfig,
    pub defaults: DefaultsConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneralConfig {
    pub default_account: String,
    pub color: ColorMode,
    pub check_updates: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub concurrency: usize,
    pub proxy: String,
    pub mirror: String,
    pub user_agent_suffix: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JavaConfig {
    pub auto_download: bool,
    pub search_paths: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DefaultsConfig {
    pub min_memory_mb: u32,
    pub max_memory_mb: u32,
    pub jvm_preset: JvmPreset,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ColorMode {
    Auto,
    Always,
    Never,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum JvmPreset {
    Default,
    Aikar,
    None,
}

impl Default for MCLConfig {
    fn default() -> Self {
        MCLConfig {
            general: GeneralConfig { default_account: "".to_string(), color: ColorMode::Auto, check_updates: true },
            network: NetworkConfig { concurrency: 4, proxy: "".to_string(), mirror: "".to_string(), user_agent_suffix: "".to_string() },
            java: JavaConfig { auto_download: true, search_paths: Vec::new() },
            defaults: DefaultsConfig { min_memory_mb: 512, max_memory_mb: 2048, jvm_preset: JvmPreset::Default },
        }
    }
}