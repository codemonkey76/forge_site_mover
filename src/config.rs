use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};

use dialoguer::{Confirm, Input};
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
    pub dest_site_name: Option<String>,
    pub dest_host: Option<String>,
    pub temp_folder: Option<String>,
    pub user_name: Option<String>,
    pub isolated: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct FinalConfig {
    pub source_folder: String,
    pub forge_api_key: String,
    pub dest_server_id: String,
    pub dest_site_name: String,
    pub dest_host: String,
    pub isolated: bool,
    pub user_name: Option<String>,
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
        if let Some(destination) = args.dest_server_id {
            self.dest_server_id = Some(destination);
        }

        if let Some(dest_site_name) = args.dest_site_name {
            self.dest_site_name = Some(dest_site_name);
        }

        if let Some(dest_host) = args.dest_host {
            self.dest_host = Some(dest_host);
        }

        if let Some(source_folder) = args.source_folder {
            self.source_folder = Some(source_folder);
        }

        if let Some(api_key) = args.forge_api_key {
            self.forge_api_key = Some(api_key);
        }

        if let Some(isolated) = args.isolated {
            self.isolated = Some(isolated);
        }

        if let Some(user_name) = args.user_name {
            self.user_name = Some(user_name);
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

        if self.dest_site_name.is_none() {
            self.dest_site_name = Some(
                Input::new()
                    .with_prompt("Please enter destination site name")
                    .interact_text()
                    .map_err(AppError::InputError)?,
            );
        }

        if self.dest_host.is_none() {
            self.dest_host = Some(
                Input::new()
                    .with_prompt("Please enter destination hostname")
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

        if self.isolated.is_none() {
            let isolated = Confirm::new()
                .with_prompt("Do you want to create an isolated site?")
                .interact()
                .map_err(AppError::InputError)?;

            self.isolated = Some(isolated);

            if isolated && self.user_name.is_none() {
                self.user_name = Some(
                    Input::new()
                        .with_prompt("Please specify the username for the isolated site")
                        .interact_text()
                        .map_err(AppError::InputError)?,
                );
            }
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
            dest_host: self
                .dest_host
                .expect("destination hostname should be provided"),
            dest_site_name: self
                .dest_site_name
                .expect("destination site name should be provided"),
            temp_folder: self.temp_folder.expect("temp folder should be provided"),
            isolated: self.isolated.expect("isolated status should be provided"),
            user_name: self.user_name,
        })
    }

    fn default() -> Self {
        Config {
            source_folder: None,
            dest_server_id: None,
            dest_site_name: None,
            dest_host: None,
            forge_api_key: None,
            temp_folder: None,
            isolated: None,
            user_name: None,
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
