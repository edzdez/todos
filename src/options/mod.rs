use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = "A simple cli tool to keep track of your todos")]
pub struct Options {
    /// Location of the todos file
    #[structopt(short = "f", long = "file")]
    pub todo_list_location: Option<PathBuf>,

    /// Select the program mode
    #[structopt(subcommand)]
    pub mode: AppMode,
}

#[derive(StructOpt, Debug)]
pub enum AppMode {
    List {
        /// Number of TODOs to show
        #[structopt(short = "s", long = "show", default_value = "5")]
        num_to_show: usize,
    },
    Add,
    Remove,
}