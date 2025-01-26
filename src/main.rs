use rand::RngCore;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_input(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("file not found");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
fn get_task(tasks: Vec<String>) -> String {
    let mut rng = rand::thread_rng();
    let index = rng.next_u32() as usize % tasks.len();
    tasks.get(index).unwrap().clone()
}

fn main() {
    let tasks = read_input("tasks.txt");
    println!("{}", get_task(tasks))
}
