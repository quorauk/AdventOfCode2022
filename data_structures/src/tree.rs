#[derive(Clone)]
pub struct Tree<T> {
    pub nodes: Vec<Node<T>>,
    pub current_node: usize,
}

#[derive(Debug, Clone)]
pub struct Node<T> {
    pub elem: T,
    pub parent: Option<usize>,
}

impl<T: Clone> Tree<T> {
    pub fn new(root: T) -> Self {
        Tree {
            nodes: vec![Node {
                elem: root,
                parent: None,
            }],
            current_node: 0,
        }
    }

    pub fn get(&self, i: usize) -> Option<T> {
        if let Some(node) = self.nodes.get(i) {
            Some(node.elem.clone())
        } else {
            None
        }
    }

    pub fn change_target(&mut self, index: usize) {
        self.current_node = index;
    }

    pub fn push(&mut self, node: T) {
        self.nodes.push(Node {
            elem: node,
            parent: Some(self.current_node),
        })
    }
}
