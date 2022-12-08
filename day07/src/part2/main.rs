use std::collections::HashMap;

const TOTAL_DISK: i32 = 70000000;
const MIN_REQUIRED: i32 = 30000000;

pub struct Tree {
    pub nodes: Vec<Node>,
}

pub struct Node {
    pub parent: Option<NodeId>,
    pub nodename: String,
    pub children: HashMap<String, NodeId>,
    pub files: HashMap<String, i32>,
    pub filesize: i32,
}

#[derive(Debug, Copy, Clone)]
pub struct NodeId {
    index: usize
}

impl Tree {
    pub fn new() -> Self {
        let mut tree = Tree { nodes: Vec::new() };
        tree.append(None, String::from("/"));
        tree
    }
    
    fn push(&mut self, parent: Option<NodeId>, nodename: String, next_index: usize) -> Option<NodeId> {
        self.nodes.push(Node {
            parent,
            nodename: nodename.clone(),
            children: HashMap::new(),
            files: HashMap::new(),
            filesize: 0
        });
        let parent_node = self.get_node(parent).unwrap();
        let node_id = NodeId { index: next_index };
        parent_node.children.insert(nodename.clone(), node_id);
        return Some(node_id)
    }
    pub fn append(&mut self, parent: Option<NodeId>, nodename: String) -> Option<NodeId> {
        let next_index = self.nodes.len();
        match next_index > 0 {
            true => {
                let parent_node = self.get_node(parent).unwrap();
                match parent_node.children.contains_key(&nodename) {
                    true => return Some(parent_node.children[&nodename]),
                    false => return self.push(parent, nodename, next_index)
                };
            },
            false => return self.push(parent, nodename, next_index)
        };
        
    }
    
    pub fn get_node(&mut self, id: Option<NodeId>) -> Option<&mut Node> {
        if self.nodes.len() < 1 {
            // Cannot get node when nodes is empty; return None
            return None
        }
        match id {
            // Node id exists; return node at index id.index
            Some(id) => return Some(&mut self.nodes[id.index]),
            // Node id is None, return root node (index 0)
            None => return Some(&mut self.nodes[0])
        }
    }
    

    pub fn add_file_size(&mut self, id: Option<NodeId>, filename: String, filesize: i32) {
        let mut node = self.get_node(id).unwrap();
        if node.files.contains_key(&filename) {
            return
        }
        node.filesize += filesize;
        node.files.insert(filename, filesize);
        loop {
            match &node.parent {
                Some(parent) => {
                    let index = parent.index;
                    node = self.get_node(Some(NodeId{index})).unwrap();
                    node.filesize += filesize;
                },
                None => break
            }
        }
    }
}

fn main() {
    let opts = adventils::OptionalArgumentsAndSuchLike::args();
    // implement solutions in here somewhere
    let lines = adventils::get_input_vec(opts.input_file.clone());
    let mut tree = Tree::new();
    let mut current = Some(NodeId{index: 0});
    for line in lines {
        match line {
            _ if line.starts_with("$") => {
                let tokens: Vec<&str> = line.split(" ").collect();
                match tokens {
                    _ if tokens[1] == "ls" => {
                        // ignore it.
                    },
                    _ if tokens[1] == "cd" => {
                        let directory = tokens[2];
                        match directory {
                            _ if directory == ".." => {
                                // get parent of current.
                                current = tree.get_node(current).unwrap().parent;
                            },
                            _ =>  {
                                //  append to current and update
                                current = tree.append(current, tokens[2].to_string());
                            }
                        }
                    },
                    _ => panic!("Shit")
                }
            },
            _ if line.starts_with("dir") => {
                //  ignore it.
            },
            _ => {
                let tokens: Vec<&str> = line.split(" ").collect();
                tree.add_file_size(current, tokens[1].to_string(), tokens[0].parse().unwrap());
            }
        };
    }

    let free = TOTAL_DISK - tree.nodes[0].filesize as i32;
    let answer = tree.nodes.iter().map(|n| n.filesize).filter(|s| *s + free >= MIN_REQUIRED).min().unwrap();
    println!("Size of smallest directory which, if deleted, would result in minimum required free space: {answer}");
}
