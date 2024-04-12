/*
    binary_search tree
    This problem requires you to implement a basic interface for a binary tree
*/

//I AM NOT DON
use std::cmp::Ordering;
use std::fmt::Debug;

#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        //TODO
        // 左子树放大，右子树放小
        match &mut self.root {
            None => self.root = Some(Box::new(TreeNode::new(value))),
            Some(root) => root.insert(value),
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        // 根据大小判定往左右子树查找，左大右小
        let mut root = &self.root;
        while let Some(node) = root {
            if node.value == value {
                return true;
            } else if value > node.value {
                root = &node.left;
            } else {
                root = &node.right;
            }
        }
        return false;
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
        // 左大右小
        if value > self.value {
            match &mut self.left {
                // 放入结点
                None => self.left = Some(Box::new(TreeNode::new(value))),
                // 递归下去
                Some(node_left) => node_left.insert(value),
            }
        } else if value < self.value {
            match &mut self.right {
                // 放入结点
                None => self.right = Some(Box::new(TreeNode::new(value))),
                // 递归下去
                Some(node_right) => node_right.insert(value),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        assert_eq!(bst.search(1), false);

        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);
        // if let Some(root) = &bst.root {
        //     assert_eq!(root.value, 5);
        //     if let Some(right) = &root.right {
        //         assert_eq!(right.value, 5);
        //     }
        // }

        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        bst.insert(1);
        bst.insert(1);

        assert_eq!(bst.search(1), true);

        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            }
            None => panic!("Root should not be None after insertion"),
        }
    }
}
