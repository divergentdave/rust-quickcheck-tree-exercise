use std::cmp::Ordering;

struct Node<K, V>
where
    K: Ord,
{
    key: K,
    value: V,
    left: Option<Box<Node<K, V>>>,
    right: Option<Box<Node<K, V>>>,
}

#[derive(Default)]
pub struct Tree<K, V>
where
    K: Ord,
{
    root: Option<Node<K, V>>,
}

impl<K, V> Tree<K, V>
where
    K: Ord,
{
    pub fn insert(&mut self, key: K, value: V) {
        let mut node: &mut Node<K, V> = match &mut self.root {
            Some(ref mut n) => n,
            None => {
                self.root = Some(Node {
                    key,
                    value,
                    left: None,
                    right: None,
                });
                return;
            }
        };
        let destination = loop {
            let result = key.cmp(&node.key);
            let maybe_next_node = match result {
                Ordering::Less => &mut node.left,
                Ordering::Equal => {
                    node.value = value;
                    return;
                }
                Ordering::Greater => &mut node.right,
            };
            match maybe_next_node {
                Some(ref mut n) => node = n,
                None => break maybe_next_node,
            }
        };
        *destination = Some(Box::new(Node {
            key,
            value,
            left: None,
            right: None,
        }));
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let mut node: &Node<K, V> = match &self.root {
            Some(n) => &n,
            None => return None,
        };
        loop {
            let maybe_next_node = match key.cmp(&node.key) {
                Ordering::Less => &node.left,
                Ordering::Equal => return Some(&node.value),
                Ordering::Greater => &node.right,
            };
            match maybe_next_node {
                Some(boxed_node) => node = boxed_node.as_ref(),
                None => return None,
            }
        }
    }
}
