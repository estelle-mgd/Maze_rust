/*use std::rc::Rc;

enum Exploration {
    Explored,
    UnExplored,
}

enum Maze {
    Branch {
        label: String,
        left: Rc<Maze>,
        right: Rc<Maze>,
        status: Exploration,
    },
    Leaf {
        label: String,
    },
}

impl Maze {
    fn newLeaf(label: String) -> Self {
        Maze::Leaf { label }
    }

    fn newBranch(label: String, left: Rc<Maze>, right: Rc<Maze>) -> Self {
        Maze::Branch {
            label,
            left,
            right,
            status: Exploration::UnExplored,
        }
    }

    /*fn explore(&mut self) -> Vec<String> {
        match self {
            Maze::Branch { label, left, right, status } => {
                match status {
                    Exploration::UnExplored => {
                        *status = Exploration::Explored;
                        let mut result = vec![label.clone()];
                        result.extend(left.explore());
                        result.extend(right.explore());
                        result
                    }
                    Exploration::Explored => vec![label.clone()],
                }
            }
            Maze::Leaf { label } => vec![label.clone()],
        }
    }*/
}

fn maze() -> Rc<Maze> {
    let leaf2 = Rc::new(Maze::Leaf { label: format!("2") });
    let leaf4 = Rc::new(Maze::Leaf { label: format!("4") });
    let leaf5 = Rc::new(Maze::Leaf { label: format!("5") });
    let leaf8 = Rc::new(Maze::Leaf { label: format!("8") });

    let branch3 = Rc::new(Maze::newBranch(format!("3"), Rc::clone(&leaf4), Rc::clone(&leaf5)));
    let branch1 = Rc::new(Maze::newBranch(format!("1"), Rc::clone(&leaf2), Rc::clone(&branch3)));
    let branch7 = Rc::new(Maze::newBranch(format!("7"), Rc::clone(&leaf5), Rc::clone(&leaf8)));
    let branch6 = Rc::new(Maze::newBranch(format!("6"), Rc::clone(&branch3), Rc::clone(&branch7)));
    Rc::new(Maze::newBranch(format!("0"), Rc::clone(&branch1), Rc::clone(&branch6)))
}

/*pub fn main() {
    let mut labyrinthe = maze();
    println!("Labyrinthe créé");

    let trace = labyrinthe.explore();
    println!("{:?}", trace);
}
*/*/

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
enum Exploration {
    UnExplored,
    LeftExplored,
    Explored,
}

#[derive(Debug, Clone)]
enum Maze {
    Branch {
        label: String,
        left: Rc<RefCell<Maze>>,
        right: Rc<RefCell<Maze>>,
        status: RefCell<Exploration>,
    },
    Leaf { label: String },
}

impl Maze {
    fn new_branch(label: &str, left: Rc<RefCell<Maze>>, right: Rc<RefCell<Maze>>) -> Rc<RefCell<Maze>> {
        Rc::new(RefCell::new(Maze::Branch {
            label: label.to_string(),
            left,
            right,
            status: RefCell::new(Exploration::UnExplored),
        }))
    }

    fn new_leaf(label: &str) -> Rc<RefCell<Maze>> {
        Rc::new(RefCell::new(Maze::Leaf {
            label: label.to_string(),
        }))
    }

    fn label(&self) -> String {
        match self {
            Maze::Branch { label, .. } => label.clone(),
            Maze::Leaf { label } => label.clone(),
        }
    }

    fn explore(
        maze: Rc<RefCell<Maze>>,
        mut stack: Vec<Rc<RefCell<Maze>>>,
        mut labels: Vec<String>,
    ) -> (Vec<Rc<RefCell<Maze>>>, Vec<String>) {
        match &*maze.borrow() {
            Maze::Branch { label, left, right, status } => {
                let current_status = status.borrow().clone();
                match current_status {
                    Exploration::UnExplored => {
                        // Transition to LeftExplored
                        {
                            let mut status_mut = status.borrow_mut();
                            *status_mut = Exploration::LeftExplored;
                        }
                        stack.push(maze.clone());
                        Maze::explore(left.clone(), stack, {
                            labels.push(label.clone());
                            labels
                        })
                    }
                    Exploration::LeftExplored => {
                        // Transition to Explored
                        {
                            let mut status_mut = status.borrow_mut();
                            *status_mut = Exploration::Explored;
                        }
                        Maze::explore(right.clone(), stack, {
                            labels.push(label.clone());
                            labels
                        })
                    }
                    Exploration::Explored => {
                        labels.push(label.clone());
                        (stack, labels)
                    }
                }
            }
            Maze::Leaf { label } => {
                labels.push(label.clone());
                (stack, labels)
            }
        }
    }
}

fn loop_explore(pair: (Vec<Rc<RefCell<Maze>>>, Vec<String>)) -> (Vec<Rc<RefCell<Maze>>>, Vec<String>) {
    let (mut stack, mut labels) = pair;
    while let Some(maze) = stack.pop() {
        println!("Exploring node {}", maze.borrow().label());
        let (new_stack, new_labels) = Maze::explore(maze, stack, labels);
        stack = new_stack;
        labels = new_labels;
    }
    (stack, labels)
}

pub fn main() {
    let leaf2 = Maze::new_leaf("2");
    let leaf4 = Maze::new_leaf("4");
    let leaf5 = Maze::new_leaf("5");
    let leaf8 = Maze::new_leaf("8");

    let branch3 = Maze::new_branch("3", leaf4.clone(), leaf5.clone());
    let branch1 = Maze::new_branch("1", leaf2.clone(), branch3.clone());
    let branch7 = Maze::new_branch("7", leaf5.clone(), leaf8.clone());
    let branch6 = Maze::new_branch("6", branch3.clone(), branch7.clone());
    let branch0 = Maze::new_branch("0", branch1.clone(), branch6.clone());

    let (_, labels) = loop_explore((vec![branch0.clone()], vec![]));
    println!("Explored nodes: {:?}", labels);
}
