use std::{collections::HashMap, hash::Hash, fmt::Display, mem::discriminant, fs::File, io::Write};

use itertools::Itertools;
use nom::{IResult, sequence::separated_pair, character::complete::{alpha1, char, line_ending}, multi::separated_list1};


trait IsUppercase {
    fn is_uppercase(&self) -> bool;
}

impl IsUppercase for String {
    fn is_uppercase(&self) -> bool {
        self.to_ascii_uppercase() == *self
    }
}


struct Graph<T>(HashMap<T,Vec<T>>);

impl<T> Graph<T>
where
    T: Hash + Eq + Clone
{
    fn new() -> Self {
        Graph(HashMap::new())
    }

    fn add_one_way_relation(&mut self, start: T, end: T) {
        self.0.entry(start).or_insert_with(Vec::new).push(end);
    }

    fn add_relation(&mut self, a: T, b: T) {
        self.add_one_way_relation(a.clone(), b.clone());
        self.add_one_way_relation(b, a);
    }

    fn get_relations(&self, node: &T) -> Option<&[T]> {
        Some(&self.0.get(node)?[..])
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
enum Node {
    Start,
    End,
    Large(String),
    Small(String),
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Node::Start => "start".to_string(),
            Node::End => "end".to_string(),
            Node::Large(s) => format!("Large({})", s),
            Node::Small(s) => format!("Small({})", s),
        })
    }
}

impl<T> From<Vec<(T, T)>> for Graph<T>
where
    T: Hash + Eq + Clone
{
    fn from(input: Vec<(T, T)>) -> Self {
        let mut out = Graph::new();
        for (a, b) in input {
            out.add_relation(a, b);
        }
        out
    }
}


enum Tree<T> {
    Branch(Vec<Tree<T>>, T),
    Leaf(T)
}

impl<T> Tree<T> {
    fn count_leaves(&self) -> u32 {
        match self {
            Tree::Branch(children, _) => {
                children.iter()
                    .map(|c| c.count_leaves())
                    .sum()
            },
            Tree::Leaf(_) => 1,
        }
    }
}

impl<T> Display for Tree<T>
where
    T: Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tree::Branch(children, n) => {
                let children: String = children.iter()
                    .map(|c| format!("{}\n", c))
                    .collect();
                let children: String = children.lines()
                    .map(|s| format!("  {}", s))
                    .intersperse("\n".into())
                    .collect();

                write!(f, "{}\n{}", n, children)
            }
            Tree::Leaf(n) => write!(f, "{}", n),
        }
    }
}


impl Graph<Node> {
    fn to_tree(&self) -> Tree<Node> {
        let mut visited = Vec::new();
        self.to_tree_recursive(Node::Start, &mut visited)
            .expect("No valid path from start to end")
    }

    fn to_tree_recursive(&self, current: Node, visited: &mut Vec<Node>) -> Option<Tree<Node>> {
        if current == Node::End {
            return Some(Tree::Leaf(current))
        }

        let connected = self.get_relations(&current)?;
        let valid_connections: Vec<_> = connected.iter()
            .filter(|n| {
                if let Node::Large(_) = n { true }
                else { !visited.contains(n) }
            })
            .collect();
        
        if valid_connections.is_empty() {
            None
        } else {
            visited.push(current.clone());
            let out = Some(Tree::Branch(valid_connections.iter()
                .map(|&n| self.to_tree_recursive(n.clone(), visited))
                .flatten()
                .collect(), current));
            visited.pop();
            out
        }
    }

    fn to_tree_small_twice(&self) -> Tree<Node> {
        let mut visited = Vec::new();
        self.to_tree_small_twice_recursive(Node::Start, &mut visited, false)
            .expect("No valid path from start to end")
    }

    fn to_tree_small_twice_recursive(
        &self, 
        current: Node, 
        visited: &mut Vec<Node>, 
        mut used_small: bool
    ) -> Option<Tree<Node>> {
        if current == Node::End {
            return Some(Tree::Leaf(current))
        }

        if let Node::Small(_) = current {
            if visited.contains(&current) {
                used_small = true;
            }
        }

        let connected = self.get_relations(&current)?;
        let valid_connections: Vec<_> = connected.iter()
            .filter(|n| match n {
                Node::Large(_) => true,
                Node::Small(_) if !used_small => true,
                _ => !visited.contains(n)
            })
            .collect();
        
        if valid_connections.is_empty() {
            None
        } else {
            visited.push(current.clone());
            let out = Some(Tree::Branch(valid_connections.iter()
                .map(|&n| self.to_tree_small_twice_recursive(n.clone(), visited, used_small))
                .flatten()
                .collect(), current));
            visited.pop();
            out
        }
    }
}


fn parse_node(input: &str) -> Node {
    match input {
        "start" => Node::Start,
        "end" => Node::End,
        n if n.to_string().is_uppercase() => Node::Large(n.into()),
        n => Node::Small(n.into()),
    }
}

fn parse_node_pair(input: &str) -> IResult<&str, (Node, Node)> {
    let (out, (a, b)) = separated_pair(alpha1, char('-'), alpha1)(input)?;
    Ok((out, (parse_node(a), parse_node(b))))
}

fn parse_graph(input: &str) -> IResult<&str, Vec<(Node, Node)>> {
    separated_list1(line_ending, parse_node_pair)(input)
}


fn count_small_paths(input: &Tree<Node>) -> u32 {
    match input {
        Tree::Branch(children, node) => {
            children.iter().map(|c| if let Node::Small(_) = node {
                c.count_leaves()
            } else {
                count_small_paths(c)
            }).sum()
        },
        Tree::Leaf(_) => 0,
    }
}

fn problem_1(input: Graph<Node>) -> u32 {
    let tree = input.to_tree();
    count_small_paths(&tree)
}

fn problem_2(input: Graph<Node>) -> u32 {
    let tree = input.to_tree_small_twice();
    tree.count_leaves()
}

fn main() {
    let input = include_str!("data.txt");
    let (_, input) = parse_graph(input).unwrap();

    let graph: Graph<Node> = input.into();

    // let out = problem_1(graph);
    let out = problem_2(graph);

    println!("{}", out);
}
