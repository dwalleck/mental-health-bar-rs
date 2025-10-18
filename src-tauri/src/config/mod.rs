use serde::{Deserialize, Serialize};

/// Application configuration using confy
#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    /// UI theme (light or dark)
    pub theme: Theme,

    /// Enable notifications
    pub notification_enabled: bool,

    /// Data retention in days (None = keep forever)
    pub data_retention_days: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    Light,
    Dark,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            theme: Theme::Light,
            notification_enabled: true,
            data_retention_days: None, // Keep all data by default
        }
    }
}

impl AppConfig {
    /// Load configuration from file (creates default if doesn't exist)
    pub fn load() -> anyhow::Result<Self> {
        let config = confy::load("mental-health-tracker", "config")?;
        Ok(config)
    }

    /// Save configuration to file
    pub fn save(&self) -> anyhow::Result<()> {
        confy::store("mental-health-tracker", "config", self)?;
        Ok(())
    }
}
