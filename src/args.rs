use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "Forge move",
    version = "1.0",
    author = "Shane Poppleton <shane@bjja.com.au>",
    about = "Automates site migrations"
)]

pub struct Args {
    /// Forge API Key
    #[arg(long, value_name = "FORGE_API_KEY")]
    pub forge_api_key: Option<String>,

    /// Destination server ID
    #[arg(long, value_name = "DEST_SERVER")]
    pub dest_server_id: Option<String>,

    /// Destination hostname
    #[arg(long, value_name = "DEST_HOST")]
    pub dest_host: Option<String>,

    /// Destination site name
    #[arg(long, value_name = "DEST_SITE_NAME")]
    pub dest_site_name: Option<String>,

    /// Source folder
    #[arg(long, value_name = "SOURCE_FOLDER")]
    pub source_folder: Option<String>,

    /// Temp folder
    #[arg(long, value_name = "TEMP_FOLDER")]
    pub temp_folder: Option<String>,

    // Create isolated environment
    #[arg(long)]
    pub isolated: Option<bool>,

    // Username to use on destination server
    #[arg(long, value_name = "USER_NAME")]
    pub user_name: Option<String>,
}
