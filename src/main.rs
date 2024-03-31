use structopt::StructOpt;
mod tasks;
mod cli;

fn main() {
    let action = cli::CommandLineArgs::from_args();
    println!("{:#?}", action);
}