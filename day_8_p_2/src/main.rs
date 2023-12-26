//! DISCLAIMER: this does not work and it is just here for me to work on it
//! when I feel like it again.

use std::collections::HashMap;
use std::str::FromStr;
use std::{cell::RefCell, rc::Rc};

const INPUT: &str = include_str!("../input.txt");
const START_NODE_NAME: &str = "AAA";
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
    /// Returns a vector of the nodes, each wrapped in `Rc<RefCell<T>>`.
    fn nodes_from_lines(lines: &[&str]) -> Vec<Rc<RefCell<Self>>> {
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

        /* Rc::clone(
            nodes
                .iter()
                .find(|node| node.borrow().name == START_NODE_NAME)
                .unwrap(),
        ) */

        nodes
    }

    /// Traverses from the starting node until `TERMINATION_NODE_NAME` is reached.
    /// After this, the program returns the amount of steps it took. THe "original"
    /// naming scheme is just because we have two versions of this.
    fn steps_until_original_termination(
        starting_node: Rc<RefCell<Self>>,
        directions: &[Direction],
    ) -> u64 {
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

    /// Traverses the nodes until the last character is the same as the
    /// last character in TERMINATION_NODE_NAME for all nodes.
    fn steps_until_alternative_termination(
        starting_nodes: Vec<Rc<RefCell<Self>>>,
        directions: &[Direction],
    ) -> u64 {
        let mut nodes_remaining = starting_nodes;
        let mut steps_needed_for_each = Vec::new();
        let mut current_steps = 0;

        loop {
            for direction in directions {
                // Go ahead and increment current steps before checking for termination.
                current_steps += 1;

                nodes_remaining.retain_mut(|node| {
                    // We go ahead and set the current node to the new node.
                    let next_node = match direction {
                        Direction::Left => Rc::clone(node.borrow().left.as_ref().unwrap()),
                        Direction::Right => Rc::clone(node.borrow().right.as_ref().unwrap()),
                    };

                    // We record whether we need to remove the new node from remaining nodes.
                    let terminator_met = next_node.borrow().name.chars().nth(2)
                        == TERMINATION_NODE_NAME.chars().nth(2);

                    *node = next_node;

                    // If we do need to remove it, we record the steps it took to
                    // get here
                    if terminator_met {
                        steps_needed_for_each.push(current_steps);
                    }

                    !terminator_met
                });

                /* for i in 0..nodes_remaining.len() {
                    let current_node = &nodes_remaining[i];

                    if current_node.borrow().name.chars().nth(2)
                        == TERMINATION_NODE_NAME.chars().nth(2)
                    {

                        continue;
                    }

                    let next_node = match direction {
                        Direction::Left => Rc::clone(current_node.borrow().left.as_ref().unwrap()),
                        Direction::Right => {
                            Rc::clone(current_node.borrow().right.as_ref().unwrap())
                        }
                    };

                    current_nodes[i] = next_node;
                } */

                // If all of the new nodes end in TERMINATION_NODE_NAME[2], then
                // we terminate.
                /* let nodes_that_havent_terminated = current_nodes.iter().filter(|current_node| {
                    //dbg!(&current_node.borrow().name);

                    current_node.borrow().name.chars().nth(2)
                        != TERMINATION_NODE_NAME.chars().nth(2)
                }); */

                /*  if nodes_that_havent_terminated.clone().count() != 6 {
                    dbg!("aaaa");
                    dbg!(nodes_that_havent_terminated.clone().count());
                }; */

                if nodes_remaining.is_empty() {
                    return steps_needed_for_each.into_iter().product();
                }
            }
        }
    }
}

fn main() {
    // Part 1
    let lines = INPUT.lines().collect::<Vec<&str>>();

    let directions = lines
        .first()
        .unwrap()
        .chars()
        .filter_map(|c| Direction::from_str(&c.to_string()).ok())
        .collect::<Vec<Direction>>();

    let nodes = Node::nodes_from_lines(&lines[2..]);

    // We currently only need to hold onto the first node (the AAA node).
    let starting_node = Rc::clone(
        nodes
            .iter()
            .find(|node| node.borrow().name == START_NODE_NAME)
            .unwrap(),
    );

    let steps_until_termination =
        Node::steps_until_original_termination(starting_node, &directions);

    println!("Steps Until Termination: {steps_until_termination}");

    // Part 2
    let nodes_that_end_with_a = nodes
        .into_iter()
        .filter(|node| node.borrow().name.chars().nth(2) == START_NODE_NAME.chars().nth(2))
        .collect::<Vec<_>>();

    let steps_until_alternative_termination =
        Node::steps_until_alternative_termination(nodes_that_end_with_a, &directions);

    println!("Steps Until Alternative Termination: {steps_until_alternative_termination}");
}
