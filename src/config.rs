use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};

use dialoguer::Input;
use serde::{Deserialize, Serialize};

use crate::{
    args::Args,
    error::{AppError, AppResult},
};

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub source_folder: Option<String>,
    pub forge_api_key: Option<String>,
    pub dest_server_id: Option<String>,
    pub isolated: bool,
}

//#[derive(Deserialize, Serialize, Debug)]
//pub struct DatabaseConfig {
//    pub name: String,
//    pub user: String,
//    pub password: String,
//}

impl Config {
    pub fn load() -> AppResult<Self> {
        // Determine configuration path
        let config_path = dirs::config_dir()
            .ok_or_else(|| {
                AppError::FileError(
                    PathBuf::new(),
                    std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        "Could not determine configuration directory",
                    ),
                )
            })?
            .join("forge_move")
            .join("config.toml");

        // Ensure config file exists or create with defaults
        if !config_path.exists() {
            println!(
                "Configuration file not found, creating one with default values at: {:?}",
                config_path
            );

            // Create the parent directory if it doesn't exist
            if let Some(parent) = config_path.parent() {
                fs::create_dir_all(parent)
                    .map_err(|e| AppError::FileError(parent.to_path_buf(), e))?;
            }

            // Generate and save default configuration
            let default_config = Config::default();
            default_config.save_to_file(&config_path)?;
            println!("Default configuration file created.");
        }

        // Load and return the configuration
        Config::from_file(&config_path)
    }

    pub fn from_args(mut self, args: Args) -> Self {
        if let Some(destination) = args.destination_server_id {
            self.dest_server_id = Some(destination);
        }

        if let Some(source_folder) = args.source_folder {
            self.source_folder = Some(source_folder);
        }

        if let Some(api_key) = args.forge_api_key {
            self.forge_api_key = Some(api_key);
        }

        if let Some(isolated) = args.isolated {
            self.isolated = isolated;
        }

        self
    }

    pub fn finalize(mut self) -> AppResult<Self> {
        if self.forge_api_key.is_none() {
            self.forge_api_key = Some(
                Input::new()
                    .with_prompt("Please enter your forge api key")
                    .interact_text()
                    .map_err(AppError::InputError)?,
            );
        }

        if self.source_folder.is_none() {
            self.source_folder = Some(
                Input::new()
                    .with_prompt("Please enter source folder")
                    .interact_text()
                    .map_err(AppError::InputError)?,
            );
        }

        Ok(self)
    }

    fn default() -> Self {
        Config {
            source_folder: None,
            dest_server_id: None,
            forge_api_key: None,
            isolated: false,
        }
    }

    fn save_to_file(&self, path: &PathBuf) -> AppResult<()> {
        let toml_content = toml::to_string(self)
            .map_err(|e| AppError::ConfigSerializationError(path.clone(), e.into()))?;
        let mut file =
            fs::File::create(path).map_err(|e| AppError::ConfigReadError(path.clone(), e))?;
        file.write_all(toml_content.as_bytes())
            .map_err(|e| AppError::FileError(path.clone(), e))
    }

    fn from_file(path: &Path) -> AppResult<Self> {
        let config_content = fs::read_to_string(path)
            .map_err(|e| AppError::ConfigReadError(path.to_path_buf(), e))?;
        let config: Config = toml::from_str(&config_content)?;
        Ok(config)
    }
}
