use std::collections::VecDeque;

pub struct NaiveAcBuilder<T> {
    nodes: Vec<Node<T>>,
    root: usize,
}

type NaiveAcInner<T> = NaiveAcBuilder<T>;

pub struct NaiveAc<T> {
    inner: NaiveAcInner<T>,
}

impl<T: AsRef<[u8]>> NaiveAc<T> {
    pub fn replace(&self, text: &[u8]) -> Vec<u8> {
        self.inner.replace(text)
    }
}

struct Node<T> {
    value: Option<T>,
    next: Vec<usize>,
    len: usize,
    fail: usize,
}

impl<T> Node<T> {
    fn dummy() -> Self {
        Node {
            value: None,
            next: vec![],
            len: 0,
            fail: 0,
        }
    }

    fn new(value: Option<T>) -> Self {
        Node {
            value,
            next: vec![0; 256],
            len: 0,
            fail: 0,
        }
    }

    fn has_edg(&self, edg: usize) -> bool {
        self.next_node(edg).is_some()
    }

    fn next_node(&self, edg: usize) -> Option<usize> {
        if self.next[edg] == 0 {
            None
        } else {
            Some(self.next[edg])
        }
    }
}

impl<T: AsRef<[u8]>> NaiveAcBuilder<T> {
    fn replace(&self, text: &[u8]) -> Vec<u8> {
        let mut result = vec![];
        let mut n = self.root;
        let mut idx = 0;
        let mut tar: Option<(usize, usize)> = None;
        let mut last = 0;
        while idx < text.len() {
            if self.nodes[n].value.is_some() {
                tar = Some((idx, n));
            }
            let edg = text[idx] as usize;
            if let Some(nn) = self.nodes[n].next_node(edg) {
                idx += 1;
                n = nn;
            } else if let Some((end, va)) = tar.take() {
                let start = end - self.nodes[va].len;
                result.extend_from_slice(&text[last..start]);
                result.extend_from_slice(self.nodes[va].value.as_ref().expect("bug!").as_ref());
                last = end;
                idx = end;
                n = self.root;
            } else if n != self.root {
                while n != self.root && !self.nodes[n].has_edg(edg) {
                    n = self.nodes[n].fail;
                }
            } else {
                idx += 1;
            }
        }
        if let Some((end, va)) = tar.take() {
            let start = end - self.nodes[va].len;
            result.extend_from_slice(&text[last..start]);
            result.extend_from_slice(self.nodes[va].value.as_ref().expect("bug!").as_ref());
            result.extend_from_slice(&text[end..]);
        } else {
            result.extend_from_slice(&text[last..]);
        }
        result
    }

    pub fn new() -> Self {
        let mut nodes = vec![Node::dummy(), Node::new(None)];
        let root = nodes.len() - 1;
        nodes[root].fail = root;
        NaiveAcBuilder { nodes, root }
    }

    pub fn insert(&mut self, key: &[u8], value: T) {
        let mut cur = self.root;
        let mut data = key;
        while let Some((&edg, rest)) = data.split_first() {
            let edg = edg as usize;
            cur = if self.nodes[cur].next[edg] == 0 {
                let n = self.alloc_node(None);
                self.nodes[cur].next[edg] = n;
                self.nodes[n].len = self.nodes[cur].len + 1;
                n
            } else {
                self.nodes[cur].next[edg]
            };
            data = rest;
        }
        self.nodes[cur].value = Some(value);
    }

    pub fn build(mut self) -> NaiveAc<T> {
        self.build_imp();
        NaiveAc { inner: self }
    }

    fn build_imp(&mut self) {
        let mut qu = VecDeque::new();
        qu.push_back(self.root);
        while let Some(cur) = qu.pop_back() {
            for (edg, n) in self.nodes[cur]
                .next
                .clone()
                .into_iter()
                .enumerate()
                .filter(|(_, n)| *n != 0)
            {
                let mut fail = self.nodes[cur].fail;
                while !self.nodes[fail].has_edg(edg) && fail != self.root {
                    fail = self.nodes[fail].fail;
                }
                self.nodes[n].fail = fail;
                if cur != self.root {
                    if let Some(nn) = self.nodes[fail].next_node(edg) {
                        self.nodes[n].fail = nn;
                    }
                }
                qu.push_back(n);
            }
        }
    }

    fn alloc_node(&mut self, value: Option<T>) -> usize {
        self.nodes.push(Node::new(value));
        self.nodes.len() - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_naive_ac1() {
        let pattern = ["1234", "12", "2333"];
        let text = "123423123233322122";
        let mut ac = NaiveAcBuilder::new();
        for pat in &pattern {
            ac.insert(pat.as_bytes(), "*");
        }
        let ac = ac.build();
        let result = ac.replace(text.as_bytes());
        let result_str = String::from_utf8_lossy(&result);
        assert_eq!(result_str, "*23*3*22*2");
    }

    #[test]
    fn test_naive_ac2() {
        let pattern = ["1221", "12", "232"];
        let text = "122212212323122122";
        let mut ac = NaiveAcBuilder::new();
        for pat in &pattern {
            ac.insert(pat.as_bytes(), "*");
        }
        let ac = ac.build();
        let result = ac.replace(text.as_bytes());
        let result_str = String::from_utf8_lossy(&result);
        assert_eq!(result_str, "*22**3*22");
    }

    #[test]
    fn test_naive_ac3() {
        let pattern = ["12312345"];
        let text = "123123445";
        let mut ac = NaiveAcBuilder::new();
        for pat in &pattern {
            ac.insert(pat.as_bytes(), "*");
        }
        let ac = ac.build();
        let result = ac.replace(text.as_bytes());
        let result_str = String::from_utf8_lossy(&result);
        assert_eq!(result_str, "123123445");
    }
}
