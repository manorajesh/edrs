use clap::Parser;
use clap::ValueHint;

#[derive(Parser, Debug, Clone)]
#[command(
    author,
    version,
    about = "A simple text editor in Rust!",
    long_about = "A non-modal plaintext editor with saving and loading functionality."
)]
pub struct Args {
    /// Path to file
    #[clap(value_name = "FILE", value_hint = ValueHint::FilePath)]
    pub file: Option<String>,
}