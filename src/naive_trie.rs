#[derive(Clone, Default)]
pub struct NaiveTrie {
    head: Node,
} 

impl NaiveTrie {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn insert(&mut self, data: &[u8]) {
        self.head.insert(data);
    }

    pub fn remove(&mut self, data: &[u8]) {
        self.head.remove(data);
    }

    pub fn contains(&self, data: &[u8]) -> bool {
        self.head.contains(data)
    }
}

#[derive(Clone)]
struct Node {
    next: Vec<Option<Node>>,
    in_use: usize,
}

impl Default for Node {
    fn default() -> Self {
        Node {
            next: vec![None; 256],
            in_use: 0,
        }
    }
}

impl Node {
    fn new() -> Self {
        Default::default()
    }

    fn insert(&mut self, data: &[u8]) {
        if let Some((&cur, rest)) = data.split_first() {
            let in_use = &mut self.in_use;
            let next = self.next[cur as usize].get_or_insert_with(|| {
                *in_use += 1;
                Node::new()
            });
            next.insert(rest);
        }
    }

    fn remove(&mut self, data: &[u8]) {
        if let Some((&cur, rest)) = data.split_first() {
            let next_slot = &mut self.next[cur as usize];
            if let Some(next) = next_slot {
                next.remove(rest);
                if next.is_empty() {
                    *next_slot = None;
                    self.in_use -= 1;
                }
            }
        }
    }

    fn contains(&self, data: &[u8]) -> bool {
        if let Some((&cur, rest)) = data.split_first() {
            let next_slot = &self.next[cur as usize];
            if let Some(next) = next_slot {
                next.contains(rest)
            } else {
                false
            }
        } else {
            true
        }
    }

    fn is_empty(&self) -> bool {
        self.in_use == 0
    }
}