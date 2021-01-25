#[derive(Debug)]
struct BST {
    node: i32,
    left: Option<Box<BST>>,
    right: Option<Box<BST>>,
}

impl BST {
    fn new(value: i32) -> Self {
        Self {
            node: value,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, value: i32) {
        match self.node > value {
            true => match self.left {
                Some(ref mut node) => node.insert(value),
                None => self.left = Some(Box::new(BST::new(value))),
            },
            _ => match self.right {
                Some(ref mut node) => node.insert(value),
                None => self.right = Some(Box::new(BST::new(value))),
            },
        }
    }

    fn search(&mut self, value: i32) {
        match self.node == value {
            true => println!("Yes the value: {} exists in BST", value),
            _ => match self.node > value {
                true => match self.left {
                    Some(ref mut left) => {
                        left.search(value);
                    }
                    None => {
                        println!("The value: {} does not exists in the BST", value);
                    }
                },
                false => match self.right {
                    Some(ref mut left) => {
                        left.search(value);
                    }
                    None => {
                        println!("The value: {} does not exists in the BST", value);
                    }
                },
            },
        }
    }

    fn size(&mut self) -> i32 {
        if Some(self.node) == None {
            0
        } else {
            let left = match self.left {
                Some(ref mut node) => node.size(),
                None => 0,
            };

            let right = match self.right {
                Some(ref mut node) => node.size(),
                None => 0,
            };

            left + 1 + right
        }
    }
}

fn main() {
    let mut bst = BST::new(10);
    bst.insert(5);
    bst.insert(8);
    bst.insert(10);
    bst.insert(1);
    bst.insert(13);
    bst.insert(11);
    bst.insert(10);

    println!("The BST is as follows {:#?}", bst);
    bst.search(1);
    bst.search(2);
    bst.search(3);
    bst.search(6);
    bst.search(9);
    bst.search(13);

    println!("The Size of BST is as follows: {}", bst.size());
}
