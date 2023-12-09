use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

fn main() {
    let input = include_str!("./d08_input");
    let output = process(input);
    dbg!(output);
}

struct Node {
    name: String,
    left: Option<Rc<RefCell<Self>>>,
    right: Option<Rc<RefCell<Self>>>,
}

impl Node {
    fn new(name: &String) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            name: name.clone(),
            left: None,
            right: None,
        }))
    }

    fn new_with(
        name: &String,
        left: &Rc<RefCell<Self>>,
        right: &Rc<RefCell<Self>>,
    ) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            name: name.clone(),
            left: Some(Rc::clone(left)),
            right: Some(Rc::clone(right)),
        }))
    }

    fn update(&mut self, left: &Rc<RefCell<Self>>, right: &Rc<RefCell<Self>>) {
        self.left = Some(Rc::clone(left));
        self.right = Some(Rc::clone(right));
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Node")
            .field("name", &self.name)
            .field(
                "left",
                &format_args!(
                    "{}",
                    match &self.left {
                        Some(node_ref) => node_ref.borrow().name.clone(),
                        None => "empty".to_string(),
                    }
                ),
            )
            .field(
                "right",
                &format_args!(
                    "{}",
                    match &self.right {
                        Some(node_ref) => node_ref.borrow().name.clone(),
                        None => "empty".to_string(),
                    }
                ),
            )
            .finish()
    }
}

fn process(input: &str) -> String {
    let mut lines = input.lines();
    let instructions: Vec<char> = lines.next().expect("string").chars().collect();
    lines.next(); // empty line

    // Network
    let mut nodes: HashMap<String, Rc<RefCell<Node>>> = HashMap::new();

    lines.collect::<Vec<&str>>().iter().for_each(|line| {
        let mut line_split = line.split(" = ");
        let name = line_split.next().expect("str").to_string();
        let mut paths_split = line_split.next().expect("string of two").split(", ");
        let left_name = paths_split.next().expect("string").replace("(", "");
        let right_name = paths_split.next().expect("string").replace(")", "");

        nodes
            .entry(left_name.clone())
            .or_insert(Node::new(&left_name));
        nodes
            .entry(right_name.clone())
            .or_insert(Node::new(&right_name));

        let left: Rc<RefCell<Node>> = nodes.get(&left_name.clone()).expect("node").clone();
        let right: Rc<RefCell<Node>> = nodes.get(&right_name.clone()).expect("node").clone();

        nodes
            .entry(name.clone())
            .and_modify(|node| {
                node.borrow_mut().update(&left, &right);
            })
            .or_insert(Node::new_with(&name, &left, &right));
    });

    let mut walk_node: Rc<RefCell<Node>> = nodes.get("AAA").expect("node").clone();
    dbg!(&nodes, &walk_node);

    // Follow tree by instructions until node.name == "ZZZ"
    let mut steps: u64 = 0;
    let mut instructions_iter = instructions.iter().cycle();
    while walk_node.borrow().name != "ZZZ" {
        let instruction = instructions_iter.next().expect("char");
        if instruction == &'L' {
            let cur_node: Rc<RefCell<Node>> =
                Rc::clone(walk_node.borrow().left.as_ref().expect("node"));
            walk_node = cur_node;
        } else if instruction == &'R' {
            let cur_node: Rc<RefCell<Node>> =
                Rc::clone(walk_node.borrow().right.as_ref().expect("node"));
            walk_node = cur_node;
        } else {
            unreachable!("invalid instruction");
        }
        //dbg!(&instruction, &walk_node);
        steps += 1;
    }

    steps.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!("2", process(input));
    }

    #[test]
    fn test_process_loop() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!("6", process(input));
    }
}
