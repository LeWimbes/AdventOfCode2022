use std::cell::RefCell;
use std::cmp::min;
use std::collections::HashMap;
use std::rc::Rc;

fn main() {
    let input = include_str!("day07.txt");
    let lines = input.lines();

    let root = build_node(0, None);
    let mut current_dir = root.clone();

    for line in lines {
        let parts: Vec<&str> = line.split(' ').collect();
        if parts[0] == "$" {
            if parts[1] == "cd" {
                current_dir = match parts[2] {
                    "/" => root.clone(),
                    ".." => current_dir.borrow().parent.clone().unwrap(),
                    _ => current_dir.borrow().children.get(parts[2]).unwrap().clone()
                }
            }
        } else { // ls out
            let name = parts[1].to_string();
            if parts[0] == "dir" {
                let new_node = build_node(0, Some(current_dir.clone()));
                current_dir.borrow_mut().children.insert(name, new_node);
            } else {
                let size = parts[0].parse::<u32>().unwrap();
                let new_node = build_node(size, Some(current_dir.clone()));
                current_dir.borrow_mut().children.insert(name, new_node);
            }
        }
    }
    calculate_size(&root);

    task1(root.clone());
    task2(root.clone());
}

fn calculate_size(node: &NodeWrapper) -> u32 {
    let mut node = node.borrow_mut();
    node.total_size = node.size + node.children.values().fold(0, |acc, child| {
        acc + calculate_size(child)
    });
    return node.total_size;
}

fn task1(root: NodeWrapper) {
    fn collect_sizes(node: &NodeWrapper) -> u32 {
        let mut sizes = node.borrow().children.values().fold(0, |acc, child| {
            acc + collect_sizes(child)
        });
        if node.borrow().is_dir() && node.borrow().total_size <= 100000 {
            sizes += node.borrow().total_size;
        }
        return sizes;
    }

    println!("{}", collect_sizes(&root));
}

fn task2(root: NodeWrapper) {
    fn get_smallest_possible(node: &NodeWrapper, min_size: u32) -> u32 {
        let mut min = node.borrow().children.values().fold(u32::MAX, |acc, child| {
            min(acc, get_smallest_possible(child, min_size))
        });
        if node.borrow().is_dir()
            && node.borrow().total_size >= min_size
            && node.borrow().total_size < min {
            min = node.borrow().total_size;
        }
        return min;
    }

    let free = 70000000 - root.borrow().total_size;
    let to_be_cleaned = 30000000 - free;

    println!("{}", get_smallest_possible(&root, to_be_cleaned));
}

type NodeWrapper = Rc<RefCell<Node>>;

struct Node {
    size: u32,
    parent: Option<NodeWrapper>,
    children: HashMap<String, NodeWrapper>,
    total_size: u32,
}

impl Node {
    fn is_dir(&self) -> bool {
        self.size == 0
    }
}

fn build_node(size: u32, parent: Option<NodeWrapper>) -> NodeWrapper {
    Rc::new(RefCell::new(Node {
        size,
        parent,
        children: Default::default(),
        total_size: size,
    }))
}