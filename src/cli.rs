use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}


#[derive(Debug, Subcommand)]
pub enum Commands {
    Init,
    CatFile {
        #[clap(short = 'p')]
        pretty_print: bool,

        #[clap(short = 't')]
        show_type: bool,

        #[clap(short = 's')]
        show_size: bool,

        object_hash: String,
    },
    HelpCommand,
}
