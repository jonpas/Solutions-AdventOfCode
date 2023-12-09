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
    let mut walk_nodes: Vec<Rc<RefCell<Node>>> = vec![];

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

        let node = nodes
            .entry(name.clone())
            .and_modify(|node| {
                node.borrow_mut().update(&left, &right);
            })
            .or_insert(Node::new_with(&name, &left, &right));

        // Save starting nodes
        if node.borrow().name.ends_with('A') {
            walk_nodes.push(Rc::clone(node));
        }
    });

    dbg!(&nodes, &walk_nodes);

    // Follow tree for each start node to find cycle
    let step_cycles: Vec<u64> = walk_nodes
        .iter_mut()
        .map(|walk_node| {
            let mut node_steps = 0;

            let mut instructions_iter = instructions.iter().cycle();
            // Cycle is the first 'Z'
            while !walk_node.borrow().name.ends_with('Z') {
                let instruction = instructions_iter.next().expect("char");

                if instruction == &'L' {
                    let cur_node: Rc<RefCell<Node>> =
                        Rc::clone(walk_node.borrow().left.as_ref().expect("node"));
                    *walk_node = cur_node;
                } else if instruction == &'R' {
                    let cur_node: Rc<RefCell<Node>> =
                        Rc::clone(walk_node.borrow().right.as_ref().expect("node"));
                    *walk_node = cur_node;
                } else {
                    unreachable!("invalid instruction");
                }

                node_steps += 1;
            }

            node_steps
        })
        .collect();

    dbg!(&step_cycles);

    let steps = lcm(&step_cycles);
    steps.to_string()
}

fn lcm(numbers: &[u64]) -> u64 {
    if numbers.len() == 1 {
        numbers[0]
    } else {
        let a = numbers[0];
        let b = lcm(&numbers[1..]);
        a * b / gcd(a, b)
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!("6", process(input));
    }
}
