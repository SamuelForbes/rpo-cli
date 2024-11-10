use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Args {
    ///The path to the directory where your photos are stored
    #[arg(short, long = "source", default_value = ".")]
    pub source_directory: String,

    ///The path to the folder you would like them sorted to
    #[arg(short, long= "target", default_value = ".")]
    pub target_directory: String,
}