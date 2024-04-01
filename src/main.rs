
use structopt::StructOpt;
use cli::Action;
use tasks::{complete_task, get_list,add_task, Task};
use utils::set_default_route;
use anyhow::anyhow;
mod tasks;
mod cli;
mod utils;




fn main() -> anyhow::Result<()> {
    let comands = cli::CommandLineArgs::from_args();

    let route = comands.route_file
        //case None from comands.route_file (default)
        .or_else(set_default_route)
        //case None from set_default_route
        .ok_or(anyhow!("Failed To find Journal File"))?;
    
    match  comands.action{
         Action::Add{task} => add_task(route, Task::new(task)),

         Action::List => get_list(route),

         Action::Done { position } => complete_task(route, position)

    }?;
    return Ok(())
}