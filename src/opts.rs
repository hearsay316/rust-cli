use clap::Parser;
use std::env;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(version, about, name = "cli")]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Debug, Parser)]
pub enum Commands {
    #[command(name = "csv", about = take())]
    Csv(CsvOpts),
}
pub fn take() -> String {
    // println!("{}", env::consts::OS);
    format!("转化成Json1{}", env::consts::OS)
}
#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long,value_parser=verify_file_exists)]
    pub input: String,

    #[arg(short, long, default_value = "output.csv")]
    pub output: String,

    #[arg(short, long, default_value = "email")] // "output.json".into()
    pub keys: String,
    #[arg(short, long, default_value_t = ',')]
    delimiter: char,

    #[arg(long, default_value_t = true)]
    header: bool,
}
pub fn verify_file_exists(file_name: &str) -> Result<String, &'static str> {
    if Path::new(file_name).exists() {
        Ok(file_name.to_string())
    } else {
        Err("没有文件")
    }
}
