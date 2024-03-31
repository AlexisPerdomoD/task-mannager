use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    /// Write tasks to the journal file.
    Add {
        /// The task description text.
        #[structopt()]
        task: String,
    },
    /// Remove an entry from the journal file by position.
    Done {
        #[structopt()]
        position: usize,
    },
    /// List all tasks in the journal file.
    #[structopt(name="list")]
    List,
}

#[derive(Debug, StructOpt)]
// this is the title or first printed text on terminal
#[structopt(
    name = "Rusty Journal title",
    about = "A command line to-do app written in Rust"
)]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    //these come in subcomands this case using action including three / comments for more information 
    pub action: Action,

    /// Use a different route file.
    //these come in options
    #[structopt(parse(from_os_str), short, long)]
    pub route_file: Option<PathBuf>,
}