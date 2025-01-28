mod rng;
#[cfg_attr(test, double)]
use crate::rng::RngWrapper;
#[cfg(test)]
use mockall_double::double;
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

#[derive(Debug, PartialEq)]
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

fn insert_at_deepest_task(
    current_tasks: Vec<String>,
    current_nesting_depth: usize,
    mut curr_tasks: &mut RecursiveTree,
) {
    let tree_tasks = &mut RecursiveTree::from_vec(current_tasks);
    for _ in 0..current_nesting_depth {
        let len = curr_tasks.children.len();
        if len == 0 {
            break;
        }
        curr_tasks = &mut curr_tasks.children[len - 1];
    }

    curr_tasks.children.append(tree_tasks);
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
            continue;
        }
        insert_at_deepest_task(current_tasks, nesting_depth, &mut tasks);
        nesting_depth = current_nesting_depth;
        current_tasks = vec![curr_val];
    }

    insert_at_deepest_task(current_tasks, nesting_depth, &mut tasks);
    tasks
}

/// get a random task from a recursive list of tasks
pub fn get_task(tasks: &RecursiveTree, top: bool) -> String {
    // the base of the tree only has one node
    let index = if top {
        0
    } else {
        RngWrapper::next_u32() as usize % tasks.children.len()
    };
    let t = &tasks.children[index];
    if t.children.len() == 0 {
        return t.value.clone();
    } else {
        get_task(t, false)
    }
}

#[cfg(test)]
mod test {
    mod build_nested_task_list {
        use crate::build_nested_task_list;

        #[test]
        fn works_for_empty_input() {
            insta::assert_debug_snapshot!(build_nested_task_list(vec![], ""));
        }
        #[test]
        fn works_for_single_nesting() {
            let input = vec![
                String::from("chore"),
                String::from(" fold"),
                String::from(" wash"),
                String::from(" clean"),
            ];

            insta::assert_debug_snapshot!(build_nested_task_list(input, " "));
        }
        #[test]
        fn works_for_multiple_nested_sections() {
            let input = vec![
                String::from("chore"),
                String::from(" fold"),
                String::from(" wash"),
                String::from(" clean"),
                String::from("fun"),
                String::from(" code"),
            ];

            insta::assert_debug_snapshot!(build_nested_task_list(input, " "));
        }
        #[test]
        fn works_for_arbitrary_nesting() {
            let input = vec![
                String::from("chore"),
                String::from(" clean"),
                String::from("  bedroom"),
                String::from("  kitchen"),
            ];
            insta::assert_debug_snapshot!(build_nested_task_list(input, " "));
        }
    }
    mod get_task {

        use crate::rng::MockRngWrapper;
        use crate::{build_nested_task_list, get_task};
        use std::sync::Mutex;

        static MTX: Mutex<()> = Mutex::new(());
        #[test]
        fn returns_a_top_level_task() {
            let _m = MTX.lock();
            let input = vec![String::from("chore"), String::from("fun")];
            let input = build_nested_task_list(input, " ");
            let ctx = MockRngWrapper::next_u32_context();
            ctx.expect().returning(|| 0);
            let res = get_task(&input, true);
            assert_eq!(res, "chore");
        }

        #[test]
        fn returns_a_nested_task() {
            let _m = MTX.lock();
            let input = vec![
                String::from("chore"),
                String::from(" clean"),
                String::from("  bedroom"),
                String::from("  kitchen"),
            ];
            let input = build_nested_task_list(input, " ");
            let ctx = MockRngWrapper::next_u32_context();
            ctx.expect().returning(|| 0);
            let res = get_task(&input, true);
            assert_eq!(res, "bedroom");
        }
    }
}
