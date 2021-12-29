use std::path::PathBuf;
use structopt::clap::arg_enum;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Options {
    /// Number of TODOs to show
    #[structopt(short = "s", long = "show", default_value = "5")]
    pub num_to_show: usize,

    /// Location of the todos file
    #[structopt(short = "f", long = "file")]
    pub todo_list_location: Option<PathBuf>,

    /// Select the program mode
    #[structopt(possible_values = &AppMode::variants(), case_insensitive = true)]
    pub mode: AppMode,
}

arg_enum! {
    #[derive(Debug)]
    pub enum AppMode {
        List,
        Add,
        Remove,
    }
}