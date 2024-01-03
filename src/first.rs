use std::mem;

pub struct List {
    head: Link,
}

impl List {
    pub fn new() -> Self {
        Self {
            head: Link::default(),
        }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            next: mem::take(&mut self.head),
        });
        self.head = Link::More(new_node)
    }
}

impl Default for List {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Default)]
enum Link {
    #[default]
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}
