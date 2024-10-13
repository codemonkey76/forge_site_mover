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
    #[arg(short, long, value_name = "FORGE_API_KEY")]
    pub forge_api_key: Option<String>,

    /// Destination server ID
    #[arg(short, long, value_name = "DESTINATION_SERVER")]
    pub destination_server_id: Option<String>,

    /// Source folder
    #[arg(short, long, value_name = "SOURCE_FOLDER")]
    pub source_folder: Option<String>,

    /// Temp folder
    #[arg(short, long, value_name = "TEMP_FOLDER")]
    pub temp_folder: Option<String>,

    // Create isolated environment
    #[arg(long)]
    pub isolated: Option<bool>,
}
