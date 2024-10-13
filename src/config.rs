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
    pub temp_folder: Option<String>,
    pub isolated: bool,
}

#[derive(Debug)]
pub struct FinalConfig {
    pub source_folder: String,
    pub forge_api_key: String,
    pub dest_server_id: String,
    pub isolated: bool,
    pub temp_folder: String,
}

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

        if let Some(temp_folder) = args.temp_folder {
            self.temp_folder = Some(temp_folder);
        }

        self
    }

    pub fn finalize(mut self) -> AppResult<FinalConfig> {
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

        if self.dest_server_id.is_none() {
            self.dest_server_id = Some(
                Input::new()
                    .with_prompt("Please specify destination server ID")
                    .interact_text()
                    .map_err(AppError::InputError)?,
            );
        }

        if self.temp_folder.is_none() {
            self.temp_folder = Some(
                Input::new()
                    .with_prompt("Please specify temp folder")
                    .interact_text()
                    .map_err(AppError::InputError)?,
            );
        }

        Ok(FinalConfig {
            source_folder: self
                .source_folder
                .expect("source folder should be specified"),
            forge_api_key: self
                .forge_api_key
                .expect("forge api key should be provided"),
            dest_server_id: self
                .dest_server_id
                .expect("destination server id should be provided"),
            temp_folder: self.temp_folder.expect("temp folder must be provided"),
            isolated: self.isolated,
        })
    }

    fn default() -> Self {
        Config {
            source_folder: None,
            dest_server_id: None,
            forge_api_key: None,
            temp_folder: None,
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
