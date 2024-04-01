use std::path::PathBuf;
use structopt::StructOpt;
use tasks::{add_task, Task};
use cli::Action;

use crate::tasks::{complete_task, get_list};
mod tasks;
mod cli;




fn main() {
    let comands = cli::CommandLineArgs::from_args();
    //temporary default route 
    let default_route:PathBuf = PathBuf::from("data.json");
    let route = comands.route_file.unwrap_or(default_route);
    
    match  comands.action{
         Action::Add{task} => add_task(route, Task::new(task)),

         Action::List => get_list(route),

         Action::Done { position } => complete_task(route, position)

    }
    //temporally error management
    .expect("faile to peformace");
}