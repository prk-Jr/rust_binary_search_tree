#![warn(unused_variables)]
#![warn(dead_code)]
use std::fmt::Display;
#[derive(Debug)]
struct BST<T> {
    node: T,
    left: Option<Box<BST<T>>>,
    right: Option<Box<BST<T>>>,
}

impl<T: Display + Ord + Copy> BST<T> {
    fn new(value: T) -> Self {
        Self {
            node: value,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, value: T) {
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

    fn search(&mut self, value: T) -> bool {
        match self.node == value {
            true => true,
            _ => match self.node > value {
                true => match self.left {
                    Some(ref mut left) => left.search(value),
                    None => false,
                },
                false => match self.right {
                    Some(ref mut left) => left.search(value),
                    None => false,
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

    fn print_inorder(&self) {
        print!("[");
        self.inorder(|x| print!("{}, ", x));
        print!("]\n")
    }

    fn inorder(&self, f: fn(&T)) {
        if let Some(ref x) = self.left {
            (*x).inorder(f);
        }
        f(&self.node);
        if let Some(ref x) = self.right {
            (*x).inorder(f);
        }
    }

    fn delete(mut this: Box<BST<T>>, target: &T) -> Option<Box<BST<T>>> {
        if target < &this.node {
            if let Some(left) = this.left.take() {
                this.left = Self::delete(left, target);
            }
            return Some(this);
        }

        if target > &this.node {
            if let Some(right) = this.right.take() {
                this.right = Self::delete(right, target);
            }
            return Some(this);
        }

        assert!(target == &this.node, "Faulty Ord implementation for T");

        match (this.left.take(), this.right.take()) {
            (None, None) => None,
            (Some(left), None) => Some(left),
            (None, Some(right)) => Some(right),
            (Some(mut left), Some(right)) => {
                if let Some(mut rightmost) = left.rightmost_child() {
                    rightmost.left = Some(left);
                    rightmost.right = Some(right);
                    Some(rightmost)
                } else {
                    left.right = Some(right);
                    Some(left)
                }
            }
        }
    }

    //  Returns the rightmost child, unless the node itself is that child.
    fn rightmost_child(&mut self) -> Option<Box<BST<T>>> {
        match self.right {
            Some(ref mut right) => {
                if let Some(t) = right.rightmost_child() {
                    Some(t)
                } else {
                    let mut r = self.right.take();
                    if let Some(ref mut r) = r {
                        self.right = std::mem::replace(&mut r.left, None);
                    }
                    r
                }
            }
            None => None,
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

    bst.print_inorder();
    println!("The Size of BST is as follows: {}", bst.size());
    println!("The BST is as follows {:#?}", bst);
    let mut new = BST::delete(Box::new(bst), &8);

    println!(
        "The Size of new BST is as follows: {:?}",
        match new {
            Some(ref mut node) => node.size(),
            None => 0,
        }
    );
    /*  println!(
        "Yes the value: {} {} exists in BST",
        1,
        if bst.search(1) { "" } else { "doesn't" }
    );
    println!(
        "Yes the value: {} {} exists in BST",
        2,
        if bst.search(2) { "" } else { "doesn't" }
    );
    println!(
        "Yes the value: {} {} exists in BST",
        3,
        if bst.search(3) { "" } else { "doesn't" }
    );
    println!(
        "Yes the value: {} {} exists in BST",
        5,
        if bst.search(5) { "" } else { "doesn't" }
    );
    println!(
        "Yes the value: {} {} exists in BST",
        10,
        if bst.search(10) { "" } else { "doesn't" }
    );
    println!(
        "Yes the value: {} {} exists in BST",
        13,
        if bst.search(13) { "" } else { "doesn't" }
    );
    println!(
        "Yes the value: {} {} exists in BST",
        11,
        if bst.search(11) { "" } else { "doesn't" }
    ); */

    // println!("The Size of BST is as follows: {}", bst.size());
}
