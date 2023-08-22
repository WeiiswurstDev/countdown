// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![feature(iter_intersperse)]
#![feature(slice_range)]

use std::{collections::HashSet, fmt::Display};

#[derive(Clone, Copy, Debug)]
pub struct Operation {
    in1: u32,
    in2: u32,
    res: u32,
    op: char,
}

impl Operation {
    fn new(in1: u32, in2: u32, res: u32, op: char) -> Operation {
        return Operation { in1, in2, res, op };
    }

    fn from(in1: u32, in2: u32) -> Vec<Operation> {
        let mut ops = vec![Operation::new(in1, in2, in1 + in2, '+')];

        if in1 != 1 && in2 != 1 {
            ops.push(Operation::new(in1, in2, in1 * in2, '*'));
        }

        if in1 > in2 {
            ops.push(Operation::new(in1, in2, in1 - in2, '-'));
        } else if in2 < in1 {
            ops.push(Operation::new(in2, in1, in2 - in1, '-'));
        }
        if in1 % in2 == 0 {
            ops.push(Operation::new(in1, in2, in1 / in2, '/'));
        } else if in2 % in1 == 0 {
            ops.push(Operation::new(in2, in1, in2 / in1, '/'));
        }
        ops
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {} = {}", self.in1, self.op, self.in2, self.res)
    }
}

#[tauri::command]
fn solve_game(target: u32, inputs: Vec<u32>) -> String {
    let result = solve_recursively(target, &inputs, &vec![], &mut HashSet::new())
        .unwrap_or(String::from("No solution found."));
    format!("Solution for {target}: {result}")
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![solve_game])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn solve_recursively(
    target: u32,
    remaining: &Vec<u32>,
    previous_ops: &Vec<Operation>,
    already_seen: &mut HashSet<Vec<u32>>,
) -> Option<String> {
    if already_seen.contains(remaining) {
        return None;
    } else {
        already_seen.insert(remaining.clone());
    }

    for x in 0..remaining.len() {
        for y in ((x + 1)..remaining.len()).filter(|i| *i != x) {
            let el1 = remaining[x];
            let el2 = remaining[y];
            let mut new_remaining = remaining.clone();
            new_remaining.remove(x);
            if x < y {
                new_remaining.remove(y - 1);
            } else {
                new_remaining.remove(y);
            }

            let possible_ops = Operation::from(el1, el2);

            match possible_ops.iter().find(|&&op| op.res == target) {
                Some(op) => {
                    let mut new_ops = previous_ops.clone();
                    new_ops.push(*op);
                    return Some(ops_to_string(&new_ops));
                }
                None => {}
            }

            for op in possible_ops.iter() {
                let mut new_ops = previous_ops.clone();
                new_ops.push(*op);

                let mut remaining_with_res = new_remaining.clone();
                let index: usize = remaining_with_res.partition_point(|x| *x < op.res);
                remaining_with_res.insert(index, op.res);

                match solve_recursively(target, &remaining_with_res, &new_ops, already_seen) {
                    Some(text) => return Some(text),
                    None => {}
                }
            }
        }
    }

    None
}

fn ops_to_string(ops: &Vec<Operation>) -> String {
    ops.iter()
        .map(|op| op.to_string())
        .intersperse(" | ".to_string())
        .collect()
}
