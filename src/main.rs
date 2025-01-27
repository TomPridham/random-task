use clap::Parser;
use random_task::{build_nested_task_list, get_task, read_input};

/// Program that reads a list of tasks from a file and selects one at random
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file_name: String,
}
fn main() {
    let args = Args::parse();

    let tasks = read_input(args.file_name);
    let tab = "  ";
    println!(
        "{}",
        get_task(&build_nested_task_list(tasks.clone(), tab), true)
    )
}
