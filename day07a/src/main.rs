fn main() {
    let mut tree = Tree::new();
    let sub1 = tree.add_node(&"sub1".to_string(), 0);
    let sub2 = tree.add_node(&"sub2".to_string(), 0);
    let sub3 = tree.add_node(&"sub3".to_string(), 0);
    tree.add_file(&"foo".to_string(), &100, &sub3);
    tree.add_file(&"foo2".to_string(), &100, &sub3);
    tree.add_file(&"bar".to_string(), &100, &sub3);
    tree.add_file(&"bar2".to_string(), &100, &sub3);
    tree.add_file(&"foo3".to_string(), &100, &sub1);
    tree.print_tree();
}

// Tree Object
struct Tree {
    root: Node, // root node with id 0
    num_nodes: usize, // number of nodes in the tree
}
impl Tree {
    fn new() -> Self {
        let root: Node = Node::new(0, "root".to_string());
        let num_nodes: usize = 0;
        Self {root, num_nodes}
    }
    //adds new node to tree as a child given parent node and returns the id of the new node
    fn add_node(&mut self, name: &String, parent_id: usize) -> usize {
        self.num_nodes += 1;
        let id = self.num_nodes;
        self.root.add_node(&name, &id, &parent_id);
        return id;
    }
    fn add_file(&mut self, file_name: &String, file_size: &usize, dir_id: &usize) {
        self.root.add_file(file_name, file_size, dir_id);
    }
    fn print_tree(&self) {
        self.root.print_node(0);
    }
}

// Node Object
struct Node {
    files: Vec<File>,
    id: usize,
    name: String,
    size: usize,
    children: Vec<Node>,
}
impl Node {
    fn new(id: usize, name: String,) -> Self {
        let children: Vec<Node> = Vec::new();
        let size: usize = 0;
        let files: Vec<File> = Vec::new();
        Self{files, id, name, size, children}
    }
    fn add_node(&mut self, name: &String, id: &usize, parent_id: &usize) -> bool {
        if *parent_id == self.id {
            let node = Node::new(*id, name.to_string());
            self.children.push(node);
            return true;
        } else if self.children.len() == 0 { // check if its a leaf
            return false;
        } else {
            for child in &mut self.children {
                if child.add_node(name, id, parent_id) { return true };
            }
            return false;
        }
    }
    fn add_file(&mut self, file_name: &String, file_size: &usize, id: &usize) -> bool {
        if *id == self.id {
            self.size += file_size;
            let file: File = File::new(file_name.to_string(), *file_size);
            self.files.push(file);
            return true;
        } else if self.children.len() == 0 {
            return false;
        } else {
            for child in &mut self.children {
                if child.add_file(file_name, file_size, id) {
                    self.size += file_size;
                    return true;
                }
            }
            return false;
        }
    }

    // DFT print Node names/sizes and File Names/sizes
    fn print_node(&self, depth: usize) {
        // print dirs
        for _i in 0..depth {
            print!("\t");
        }
        println!("Dir {}: id={}, size={}", self.name, self.id, self.size);
        // print files
        for _i in 0..depth+1 {
            print!("\t");
        }
        print!("Files:\t");
        for f in &self.files {
            print!("{}\t", f.name);
        }
        println!("");
        if self.children.len() == 0 {
            return;
        } else {
            for child in &self.children {
                child.print_node(depth+1);
            }
            return
        }
    }
}

// File Object
struct File {
    name: String,
    size: usize,
}
impl File {
    fn new(name: String, size: usize) -> Self {
        Self {name, size}
    }
}