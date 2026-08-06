use crate::inventory::Inventory;
use std::collections::BTreeMap;

#[derive(Default)]
struct TreeNode {
    children: BTreeMap<String, TreeNode>,
}

fn insert(node: &mut TreeNode, path: &str) {
    let mut current = node;

    for part in path.split('/') {
        current = current.children.entry(part.to_string()).or_default();
    }
}

pub fn print(inventory: &Inventory) {
    println!("{}", inventory.project_name);

    let mut root = TreeNode::default();

    for file in &inventory.files_data {
        insert(&mut root, &file.path);
    }

    print_node(&root, "");
}

fn print_node(node: &TreeNode, prefix: &str) {
    let len = node.children.len();

    for (index, (name, child)) in node.children.iter().enumerate() {
        let last = index + 1 == len;

        let branch = if last { "└── " } else { "├── " };

        println!("{prefix}{branch}{name}");

        let next_prefix = if last {
            format!("{prefix}    ")
        } else {
            format!("{prefix}│   ")
        };

        print_node(child, &next_prefix);
    }
}
