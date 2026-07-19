pub mod graph;

use std::collections::HashMap;
use std::env::args;
use std::fs::{read_to_string, write};
use std::process::exit;
use regex::Regex;
use crate::graph::Graph;

fn split_by_labels(code: String) -> Vec<String> {
    let re = Regex::new(r"[\w\d_]+:\n").unwrap();
    let code = code.as_str();

    // 2. Collect the starting byte indexes of all matches
    let split_points: Vec<usize> = re.find_iter(code)
        .map(|m| m.start())
        .collect();

    // 3. Build the chunks using the boundaries
    let mut parts = Vec::new();
    let mut current_idx = 0;

    for &next_match_start in &split_points {
        // Skip adding an empty string if the first match is at index 0
        if next_match_start > current_idx {
            parts.push(code[current_idx..next_match_start].trim().to_string());
        }
        current_idx = next_match_start;
    }

    // 4. Push the remaining text after the final match
    if current_idx < code.len() {
        parts.push(code[current_idx..].trim().to_string());
    }

    parts
}

fn get_label(part: String) -> (String, String) {
    let parts = part.splitn(2, "\n").collect::<Vec<&str>>();
    let name = if parts[0].contains(":") {
        parts[0].to_string().replace(":", "")
    } else {
        "start".to_string()
    };
    (name, part)
}

fn prep_code(code: String) -> String {
    let mut code = code;

    let comment_regex = Regex::new(r";.*").unwrap();

    code = code.split('\n')
        .map(|line| {comment_regex.replace_all(line, "").trim().to_string()})
        .filter(|line| !line.is_empty())
        .collect::<Vec<String>>()
        .join("\n");

    code
}

fn generate_graph(code: String) -> Graph {
    let mut graph = Graph::new();

    let splitted = split_by_labels(prep_code(code));

    // Pass 1: Building nodes and connecting them to labels
    let mut label_to_id: HashMap<String, String> = HashMap::new();
    for part in &splitted {
        let (name, code) = get_label(part.clone());
        let id = graph.add_node(name.as_str(), code.as_str());
        println!("{} -> {}", name, id);
        label_to_id.insert(name, id);
    }

    // Pass 2: Connect the fucker
    for part in &splitted {
        let (name, code) = get_label(part.clone());

        for line in code.lines() {
            let target = if line.starts_with("jmp") || line.starts_with("call") {
                line.splitn(2, ' ').skip(1).next().unwrap()
            } else if line.starts_with("jz") || line.starts_with("jnz") {
                line.splitn(3, ' ').skip(2).next().unwrap()
            } else {
                continue;
            };
            let target = &target[1..];
            if label_to_id.contains_key(target) {
                graph.connect_nodes(&label_to_id[&name], &label_to_id[target]);
            }
        }

        let last_line = code.lines().last().unwrap();
        if !(last_line.starts_with("jmp") ||
            last_line.starts_with("ret"))
        {
            let current_id = label_to_id[&name].clone();
            let current_id_num = Graph::id_to_node(current_id.as_str());
            if current_id_num + 1 < graph.node_count() {
                let next_id = Graph::node_id(current_id_num + 1);
                graph.connect_nodes(&current_id, &next_id);
            }
        }
    }

    graph
}

fn main() {
    let args = args().collect::<Vec<_>>();

    if args.len() != 2 {
        println!("Usage: {} <filename>", args[0]);
        exit(1);
    }

    let code = read_to_string(args[1].clone()).unwrap();
    let graph = generate_graph(code);

    let output = graph.build();

    write("output.dot", output).unwrap();
}
