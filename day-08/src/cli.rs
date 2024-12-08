use clap::Parser as ClapParser;

#[derive(ClapParser, Debug)]
#[command(author, version = "0.0.1", about)]
pub struct Cli {
    #[arg(help = "The step of the process")]
    pub step: i32,

    #[arg(help = "The input file")]
    pub file: String,
}
