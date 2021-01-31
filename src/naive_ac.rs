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
        let mut matches: Vec<(usize, usize, _)> = vec![];
        let mut cur = self.root;
        for idx in 0..=text.len() {
            let edg = if idx == text.len() {
                0
            } else {
                text[idx] as usize
            };
            loop {
                if let Some(n) = self.nodes[cur].next_node(edg) {
                    cur = n;
                    break;
                } else {
                    if cur == self.root {
                        break;
                    }
                    cur = self.nodes[cur].fail;
                }
            }
            let mut p = cur;
            while p != self.root {
                if let Some(value) = &self.nodes[p].value {
                    matches.push((idx + 1 - self.nodes[p].len, idx + 1, value));
                }
                p = self.nodes[p].fail;
            }
        }
        matches.sort_by(|a, b| {
            if a.0 == b.0 {
                a.1.cmp(&b.1)
            } else {
                a.0.cmp(&b.0)
            }
        });
        let mut last = 0;
        let mut ret = vec![];
        let mut iter = matches.into_iter().peekable();
        while let Some(v) = iter.next() {
            let (start, end, value) = v;
            if last > start
                || iter
                    .peek()
                    .map(|&(s, e, _)| s == start && e > end)
                    .unwrap_or(false)
            {
                continue;
            }
            ret.extend_from_slice(&text[last..start]);
            ret.extend_from_slice(value.as_ref());
            last = end;
        }
        ret.extend_from_slice(&text[last..]);
        ret
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
            cur = if !self.nodes[cur].has_edg(edg) {
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
        while let Some(cur) = qu.pop_front() {
            for (edg, n) in self.nodes[cur]
                .next
                .clone()
                .into_iter()
                .enumerate()
                .filter(|(_, n)| *n != 0)
            {
                if cur == self.root {
                    self.nodes[n].fail = cur;
                    qu.push_back(n);
                    continue;
                }
                let mut fail = self.nodes[cur].fail;
                while !self.nodes[fail].has_edg(edg) && fail != self.root {
                    fail = self.nodes[fail].fail;
                }
                if let Some(nn) = self.nodes[fail].next_node(edg) {
                    self.nodes[n].fail = nn;
                } else {
                    self.nodes[n].fail = self.root;
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
    use std::io::BufRead;
    use std::io::BufReader;

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

    #[test]
    fn test_naive_ac4() {
        let text = "新变形金刚8";
        let ac = build_dict_ac().expect("fail to build ac");
        let result = ac.replace(text.as_bytes());
        let result_str = String::from_utf8_lossy(&result);
        assert_eq!(result_str, "新*8");
    }

    fn build_dict_ac() -> anyhow::Result<NaiveAc<&'static str>> {
        let dict = std::fs::File::open("ac/data/dict.txt")?;
        let reader = BufReader::new(dict);
        let mut ac = NaiveAcBuilder::new();
        for line in reader.lines() {
            let line = line?;
            let key = line
                .split_whitespace()
                .next()
                .expect("fail to parse dict line");
            ac.insert(key.as_bytes(), "*");
        }
        Ok(ac.build())
    }

    #[test]
    fn test_naive_ac5() {
        let text = "1111111";
        let mut ac = NaiveAcBuilder::new();
        ac.insert("1111".as_bytes(), "*");
        ac.insert("111".as_bytes(), "#");
        let ac = ac.build();
        let result = ac.replace(text.as_bytes());
        let result_str = String::from_utf8_lossy(&result);
        assert_eq!(result_str, "*#");
    }

    #[test]
    fn test_naive_ac6() {
        let text = "111111121112";
        let mut ac = NaiveAcBuilder::new();
        ac.insert("1111".as_bytes(), "*");
        ac.insert("1112".as_bytes(), "#");
        let ac = ac.build();
        let result = ac.replace(text.as_bytes());
        let result_str = String::from_utf8_lossy(&result);
        assert_eq!(result_str, "*##");
    }

    #[test]
    fn test_naive_ac7() {
        let text = "爱一个人其实不容易";
        let ac = build_dict_ac().expect("fail to build ac");
        let result = ac.replace(text.as_bytes());
        let result_str = String::from_utf8_lossy(&result);
        assert_eq!(result_str, "爱*其实不容易");
    }

    #[test]
    fn test_naive_ac8() {
        let text = "1234";
        let mut ac = NaiveAcBuilder::new();
        ac.insert("23".as_bytes(), "*");
        ac.insert("12345".as_bytes(), "*");
        let ac = ac.build();
        let result = ac.replace(text.as_bytes());
        let result_str = String::from_utf8_lossy(&result);
        assert_eq!(result_str, "1*4");
    }

    #[test]
    fn test_naive_ac9() {
        let text = "12345";
        let mut ac = NaiveAcBuilder::new();
        ac.insert("23".as_bytes(), "*");
        ac.insert("1234".as_bytes(), "*");
        let ac = ac.build();
        let result = ac.replace(text.as_bytes());
        let result_str = String::from_utf8_lossy(&result);
        assert_eq!(result_str, "*5");
    }
}
