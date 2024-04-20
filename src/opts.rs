use clap::Parser;

#[derive(Debug, Parser)]
#[command(name="rcli", version, author, about, long_about=None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}
#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or Convert CSV file to JSON file")]
    Csv(CsvOpts),
}
#[derive(Debug, Parser)]
pub struct CsvOpts {
    /// Input CSV file
    #[arg(short, long, value_parser = verify_input)]
    pub input: String,
    /// Output JSON file
    #[arg(short, long, default_value = "output.json")]
    pub output: String,
    /// Delimiter
    #[arg(short, long, default_value = ",")]
    pub delimiter: char,
    /// Header
    #[arg(long, default_value_t = true)]
    pub header: bool,
}

fn verify_input(s: &str) -> Result<String, String> {
    if std::path::Path::new(s).exists() {
        Ok(s.to_string())
    } else {
        Err(format!("File not found: {}", s))
    }
}
