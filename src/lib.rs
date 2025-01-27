use rand::RngCore;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_input(file_name: String) -> Vec<String> {
    let file = File::open(file_name).expect("file not found");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

#[derive(Debug)]
pub struct RecursiveTree {
    key: usize,
    value: String,
    children: Vec<RecursiveTree>,
}

impl RecursiveTree {
    fn from_vec(arr: Vec<String>) -> Vec<Self> {
        arr.into_iter()
            .enumerate()
            .map(|(key, value)| RecursiveTree {
                key,
                value,
                children: vec![],
            })
            .collect()
    }
}
pub fn build_nested_task_list(raw_tasks: Vec<String>, nesting_delimiter: &str) -> RecursiveTree {
    let mut tasks = RecursiveTree {
        key: 0,
        value: String::new(),
        children: vec![],
    };
    let mut nesting_depth = 0;
    let mut current_tasks = vec![];
    for raw_task in raw_tasks {
        let split: Vec<String> = raw_task
            .split(nesting_delimiter)
            .map(|s| s.to_string())
            .collect();
        let current_nesting_depth = split.len() - 1;
        let curr_val = split.last().unwrap().clone();
        if current_nesting_depth == nesting_depth {
            current_tasks.push(curr_val);
        } else {
            let tree_tasks = &mut RecursiveTree::from_vec(current_tasks);
            if nesting_depth == 0 {
                tasks.children.append(tree_tasks);
            } else {
                let l = tasks.children.len() - 1;
                tasks.children[l].children.append(tree_tasks);
            }
            nesting_depth = current_nesting_depth;
            current_tasks = vec![curr_val];
        }
    }
    tasks
}

/// get a random task from a recursive list of tasks
pub fn get_task(tasks: &RecursiveTree, top: bool) -> String {
    let mut rng = rand::thread_rng();

    let index = if top {
        0
    } else {
        rng.next_u32() as usize % tasks.children.len()
    };
    let t = &tasks.children[index];
    if t.children.len() == 0 {
        return t.value.clone();
    } else {
        get_task(t, false)
    }
}
