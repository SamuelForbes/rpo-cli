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
    
    ///A comma seperated list of file formats that are considered valid to check and move
    #[arg(short, long="valid-file-formats", value_parser, num_args=1.. , value_delimiter=',', default_value="jpeg,jpg,png")]
    pub valid_file_formats: Vec<String>,
}