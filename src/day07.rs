#[macro_use]
extern crate nom;

use std::collections::{HashSet, HashMap};
use std::str::FromStr;
use nom::{space, alpha, digit};


/// Node (program)
#[derive(Debug, Clone, PartialEq)]
struct Node {
    name: String,
    weight: u32,
    children: Vec<String>,
}

impl FromStr for Node {
    type Err = nom::ErrorKind;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        named!(name<&str, String>, map_res!(alpha, FromStr::from_str));
        named!(number<&str, u32>, map_res!(digit, FromStr::from_str));
        named!(namelist<&str, Vec<String>>, separated_nonempty_list_complete!(tag!(", "), name));
        complete!(s, do_parse!(
            name: name >> space >>
            weight: delimited!(tag!("("), number, tag!(")")) >>
            children: alt_complete!(preceded!(tag!(" -> "), namelist) | value!(vec![])) >>
            (Node { name: name, weight: weight, children: children })
        )).to_result()
    }
}


/// Tree of nodes (programs)
#[derive(Debug)]
struct Tree {
    root: String,
    nodes: HashMap<String, Node>,
}

impl FromStr for Tree {
    type Err = nom::ErrorKind;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nodes = HashMap::new();
        let mut names = HashSet::new();
        for line in s.lines() {
            let node: Node = try!(line.parse());
            names.insert(node.name.clone());
            nodes.insert(node.name.clone(), node);
        }
        for node in nodes.values() {
            for child in node.children.iter() {
                names.remove(child);
            }
        }
        if names.len() != 1 {
            // Error: not a single root node
            return Err(nom::ErrorKind::Custom(0));
        }
        let root = names.drain().nth(0).unwrap();
        Ok(Tree { root: root, nodes: nodes })
    }
}

impl Tree {
    /// Returns the weight of the given node (node weight only)
    fn weight(&self, name: &str) -> Option<u32> {
        self.nodes.get(name).map(|node|
            node.weight
        )
    }

    /// Calculate total weight of the given node (node weight plus children weights)
    fn total_weight(&self, name: &str) -> Option<u32> {
        self.nodes.get(name).map(|node|
            node.children.iter().fold(node.weight, |weight, child|
                weight + self.total_weight(child).unwrap()
            )
        )
    }

    /// Check children weights of the given node (and return the corrected weight)
    fn check_weights(&self, name: &str) -> Option<u32> {
        self.nodes.get(name).and_then(|node| {
            if node.children.is_empty() {
                return None;
            }
            for child in node.children.iter() {
                if let Some(w) = self.check_weights(&child) {
                    return Some(w);
                }
            }
            let mut children_weights: Vec<(u32, u32)> = node.children.iter().map(|child|
                (self.weight(child).unwrap(), self.total_weight(child).unwrap())
            ).collect();
            children_weights.sort_by_key(|&(_, w)| w);
            let median_weight = children_weights[children_weights.len() / 2];
            let weight_offsets: Vec<(u32, i32)> = children_weights.iter().map(|&weight|
                (weight.0, weight.1 as i32 - median_weight.1 as i32)
            ).filter(|&offset|
                offset.1 != 0
            ).collect();
            match weight_offsets.len() {
                0 => None,
                1 => Some((weight_offsets[0].0 as i32 - weight_offsets[0].1) as u32),
                _ => panic!("can't handle more than 1 imbalanced node"),
            }
        })
    }

    /// Check weights of all nodes
    fn check_all_weights(&self) -> Option<u32> {
        self.check_weights(&self.root)
    }
}


fn main() {
    let tree: Tree = include_str!("day07.txt").parse().unwrap();
    println!("Root program: {}", tree.root);
    println!("Correct weight of imbalanced node: {}", tree.check_all_weights().unwrap());
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        assert_eq!(Node::from_str("pbgs (66)"), Ok(Node { name: "pbgs".to_string(), weight: 66, children: vec![] }));
        assert_eq!(Node::from_str("fwft (72) -> ktlj, cntj, xhth"), Ok(Node { name: "fwft".to_string(), weight: 72, children: vec!["ktlj".to_string(), "cntj".to_string(), "xhth".to_string()] }));
        let tree: Tree = "pbga (66)\nxhth (57)\nebii (61)\nhavc (66)\nktlj (57)\nfwft (72) -> ktlj, cntj, xhth\nqoyq (66)\npadx (45) -> pbga, havc, qoyq\ntknk (41) -> ugml, padx, fwft\njptl (61)\nugml (68) -> gyxo, ebii, jptl\ngyxo (61)\ncntj (57)".parse().unwrap();
        assert_eq!(tree.nodes.len(), 13);
    }

    #[test]
    fn samples1() {
        let tree: Tree = "pbga (66)\nxhth (57)\nebii (61)\nhavc (66)\nktlj (57)\nfwft (72) -> ktlj, cntj, xhth\nqoyq (66)\npadx (45) -> pbga, havc, qoyq\ntknk (41) -> ugml, padx, fwft\njptl (61)\nugml (68) -> gyxo, ebii, jptl\ngyxo (61)\ncntj (57)".parse().unwrap();
        assert_eq!(tree.root, "tknk");
    }

    #[test]
    fn samples2() {
        let tree: Tree = "pbga (66)\nxhth (57)\nebii (61)\nhavc (66)\nktlj (57)\nfwft (72) -> ktlj, cntj, xhth\nqoyq (66)\npadx (45) -> pbga, havc, qoyq\ntknk (41) -> ugml, padx, fwft\njptl (61)\nugml (68) -> gyxo, ebii, jptl\ngyxo (61)\ncntj (57)".parse().unwrap();
        assert_eq!(tree.weight("ugml"), Some(68));
        assert_eq!(tree.weight("padx"), Some(45));
        assert_eq!(tree.weight("fwft"), Some(72));
        assert_eq!(tree.total_weight("ugml"), Some(251));
        assert_eq!(tree.total_weight("padx"), Some(243));
        assert_eq!(tree.total_weight("fwft"), Some(243));
        assert_eq!(tree.check_all_weights(), Some(60));
    }
}
