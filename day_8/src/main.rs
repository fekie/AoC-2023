use std::collections::HashMap;
use std::str::FromStr;
use std::{cell::RefCell, rc::Rc};

const INPUT: &str = include_str!("../input.txt");
const TERMINATION_NODE_NAME: &str = "ZZZ";

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            _ => Err(()),
        }
    }
}

/// Represents a `node` from the input file. After the [`Node`] is
/// initialized, you can just unwrap `left` and `right`. We need to have them
/// wrapped in `Option`s because the left and right nodes may not exist when
/// first creating a bunch of `Node`s.
#[derive(Debug)]
struct Node {
    name: String,
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
}

impl Node {
    /// Accepts an input of lines, starting from the first "node line".
    /// Returns the first starting node that connects to everything else.
    fn starting_node_from_lines(lines: &[&str]) -> Rc<RefCell<Self>> {
        // We put all the nodes in here. They may not all be finished.
        let mut nodes: Vec<Rc<RefCell<Self>>> = Vec::new();

        // We create a hashmap of the string name and the node value.
        let mut name_node_map: HashMap<String, Rc<RefCell<Self>>> = HashMap::new();

        // We keep a hashmap of node names, and the names of nodes
        // to add to the node after we are done with parsing.
        // Format: `HashMap<name, (left_name, right_name)>`
        let mut missing_nodes: HashMap<String, (String, String)> = HashMap::new();

        // We do the parsing here.
        for line in lines {
            let name = line[0..3].to_string();
            let left_name = line[7..10].to_string();
            let right_name = line[12..15].to_string();

            // We go ahead and write down the names of the nodes that
            // we'll need.
            missing_nodes.insert(name.clone(), (left_name, right_name));

            let node = Rc::new(RefCell::new(Self {
                name: name.clone(),
                left: None,
                right: None,
            }));

            // We add this node to all the maps and vectors above
            nodes.push(Rc::clone(&node));
            name_node_map.insert(name, Rc::clone(&node));
        }

        // We now "attach" all the nodes to each other.
        for node in &nodes {
            let (left_name, right_name) = missing_nodes.get(&node.borrow().name).unwrap();

            let left_node = Rc::clone(name_node_map.get(left_name).unwrap());
            let right_node = Rc::clone(name_node_map.get(right_name).unwrap());

            node.borrow_mut().left = Some(left_node);
            node.borrow_mut().right = Some(right_node);
        }

        // We only need to return the first node. The rest are still in memory
        // because they are wrapped in `Rc`.

        Rc::clone(&nodes[0])
    }

    /// Traverses from the starting node until `TERMINATION_NODE_NAME` is reached.
    /// After this, the program returns the amount of steps it took.
    fn steps_until_termination(starting_node: Rc<RefCell<Self>>, directions: &[Direction]) -> u64 {
        let mut current_node = Rc::clone(&starting_node);
        let mut steps = 0;

        loop {
            for direction in directions {
                let next_node = match direction {
                    Direction::Left => Rc::clone(current_node.borrow().left.as_ref().unwrap()),
                    Direction::Right => Rc::clone(current_node.borrow().right.as_ref().unwrap()),
                };

                current_node = next_node;

                // Go ahead and increment steps before checking for termination.
                steps += 1;

                if current_node.borrow().name == TERMINATION_NODE_NAME {
                    return steps;
                }
            }
        }
    }
}

fn main() {
    let lines = INPUT.lines().collect::<Vec<&str>>();

    let directions = lines
        .first()
        .unwrap()
        .chars()
        .filter_map(|c| Direction::from_str(&c.to_string()).ok())
        .collect::<Vec<Direction>>();

    let starting_node = Node::starting_node_from_lines(&lines[2..]);

    let steps_until_termination = Node::steps_until_termination(starting_node, &directions);

    println!("Steps Until Termination: {steps_until_termination}");
}
