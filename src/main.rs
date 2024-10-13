use std::{cmp::max, thread, time::Duration};

#[derive(Debug, Clone)]
struct Node {
    key: i32,
    height: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(key: i32) -> Self {
        Node {
            key,
            height: 1,
            left: None,
            right: None,
        }
    }
}

struct AVLTree {
    root: Option<Box<Node>>,
}

impl AVLTree {
    fn new() -> Self {
        AVLTree { root: None }
    }

    fn height(&self, node: &Option<Box<Node>>) -> i32 {
        match node {
            Some(n) => n.height,
            None => 0,
        }
    }

    fn get_balance(&self, node: &Option<Box<Node>>) -> i32 {
        match node {
            Some(n) => self.height(&n.left) - self.height(&n.right),
            None => 0,
        }
    }

    fn rotate_right(&mut self, mut y: Box<Node>) -> Box<Node> {
        let mut x = y.left.take().unwrap();
        let t2 = x.right.take();

        x.right = Some(y);
        x.right.as_mut().unwrap().left = t2;

        x.right.as_mut().unwrap().height = max(
            self.height(&x.right.as_ref().unwrap().left),
            self.height(&x.right.as_ref().unwrap().right),
        ) + 1;
        x.height = max(self.height(&x.left), self.height(&x.right)) + 1;

        x
    }

    fn rotate_left(&mut self, mut x: Box<Node>) -> Box<Node> {
        let mut y = x.right.take().unwrap();
        let t2 = y.left.take();

        y.left = Some(x);
        y.left.as_mut().unwrap().right = t2;

        y.left.as_mut().unwrap().height = max(
            self.height(&y.left.as_ref().unwrap().left),
            self.height(&y.left.as_ref().unwrap().right),
        ) + 1;
        y.height = max(self.height(&y.left), self.height(&y.right)) + 1;

        y
    }

    fn insert(&mut self, node: Option<Box<Node>>, key: i32) -> Option<Box<Node>> {
        let mut node = match node {
            Some(n) => n,
            None => return Some(Box::new(Node::new(key))),
        };

        if key < node.key {
            node.left = self.insert(node.left.take(), key);
        } else if key > node.key {
            node.right = self.insert(node.right.take(), key);
        } else {
            return Some(node);
        }

        node.height = max(self.height(&node.left), self.height(&node.right)) + 1;
        let balance = self.get_balance(&Some(Box::new(*node.clone())));

        if balance > 1 && key < node.left.as_ref().unwrap().key {
            return Some(self.rotate_right(node));
        }

        if balance < -1 && key > node.right.as_ref().unwrap().key {
            return Some(self.rotate_left(node));
        }

        if balance > 1 && key > node.left.as_ref().unwrap().key {
            node.left = Some(self.rotate_left(node.left.take().unwrap()));
            return Some(self.rotate_right(node));
        }

        if balance < -1 && key < node.right.as_ref().unwrap().key {
            node.right = Some(self.rotate_right(node.right.take().unwrap()));
            return Some(self.rotate_left(node));
        }

        Some(node)
    }

    fn insert_key(&mut self, key: i32) {
        let root = self.root.take();
        self.root = self.insert(root, key);
    }

    fn search(&self, node: &Option<Box<Node>>, key: i32) -> bool {
        match node {
            Some(n) => {
                if key == n.key {
                    true
                } else if key < n.key {
                    self.search(&n.left, key)
                } else {
                    self.search(&n.right, key)
                }
            }
            None => false,
        }
    }

    fn search_key(&self, key: i32) -> bool {
        self.search(&self.root, key)
    }
}

fn main() {
    println!("Program started, sleeping for 10 seconds before operations...");
    thread::sleep(Duration::from_secs(30));

    let mut tree = AVLTree::new();
    println!("AVL Tree created.");

    println!("Inserting 10 into the AVL tree.");
    tree.insert_key(10);
    println!("10 inserted.");

    println!("Inserting 20 into the AVL tree.");
    tree.insert_key(20);
    println!("20 inserted.");

    println!("Inserting 30 into the AVL tree.");
    tree.insert_key(30);
    println!("30 inserted.");

    let search_result = tree.search_key(20);
    println!("Search 20: {}", search_result);
    if search_result {
        println!("20 found in the tree.");
    } else {
        println!("20 not found in the tree.");
    }

    println!("Operations completed. Sleeping for another 10 seconds.");
    thread::sleep(Duration::from_secs(10));

    println!("Program execution completed.");
}
