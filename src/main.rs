use std::path::PathBuf;

use structopt::StructOpt;

use crate::tasks::{add_task, Task};
mod tasks;
mod cli;

fn main() {
    let comands = cli::CommandLineArgs::from_args();
    println!("{:#?}", comands);
    let default_route:PathBuf = PathBuf::new();
    println!("{:?}", default_route);
    let route = comands.route_file.unwrap_or(default_route);
    
    match  comands.action{
         add => add_task(route, Task::new("prueba".to_string())),
         list => todo!(),
         Done => todo!()
        };
}