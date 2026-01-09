use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "occ-recruitment")]
#[command(author = "Open Computing Club")]
#[command(version = "0.1.0")]
#[command(about = "Generates a recruitment campaign layout with QR code for Open Computing Club", long_about = None)]
pub struct Args {
    /// URL to encode in the QR code (optional, will prompt if not provided)
    #[arg(value_name = "URL")]
    pub url: Option<String>,
}
