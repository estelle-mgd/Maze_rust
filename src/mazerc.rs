use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
enum Exploration {
    UnExplored,
    Explored,
}

#[derive(Debug, Clone)]
enum Maze {
    Branch {
        label: String,
        left: Rc<Maze>,
        right: Rc<Maze>,
        status: Rc<RefCell<Exploration>>, // Utilisation de RefCell pour rendre `status` mutable
    },
    Leaf { label: String },
}

impl Maze {
    fn new_branch(label: &str, left: Rc<Maze>, right: Rc<Maze>) -> Rc<Maze> {
        Rc::new(Maze::Branch {
            label: label.to_string(),
            left,
            right,
            status: Rc::new(RefCell::new(Exploration::UnExplored)), // Initialisation de `status` avec RefCell
        })
    }

    fn new_leaf(label: &str) -> Rc<Maze> {
        Rc::new(Maze::Leaf {
            label: label.to_string(),
        })
    }

    fn label(&self) -> String {
        match self {
            Maze::Branch { label, .. } => label.clone(),
            Maze::Leaf { label } => label.clone(),
        }
    }

    fn explore(
        maze: Rc<Maze>,
        mut stack: Vec<Rc<Maze>>,
        mut labels: Vec<String>,
    ) -> (Vec<Rc<Maze>>, Vec<String>) {
        match &*maze {
            Maze::Branch { label, left, right, status } => {
                let current_status = status.borrow().clone(); // Emprunt immuable pour lire le status
                match current_status {
                    Exploration::UnExplored => {
                        {
                            let mut status_mut = status.borrow_mut(); // Emprunt mutable pour changer le status
                            *status_mut = Exploration::Explored;
                        }
                        // Ajouter le sous-arbre droit à la pile, puis explorer le gauche
                        stack.push(right.clone());
                        stack.push(left.clone());
                        labels.push(label.clone());
                        (stack, labels)
                    }
                    Exploration::Explored => {
                        labels.push(label.clone());
                        (stack, labels)
                    },
                }
            }
            Maze::Leaf { label } => {
                labels.push(label.clone());
                (stack, labels)
            }
        }
    }
}

fn loop_explore(pair: (Vec<Rc<Maze>>, Vec<String>)) -> (Vec<Rc<Maze>>, Vec<String>) {
    let (mut stack, mut labels) = pair;
    while let Some(maze) = stack.pop() {
        println!("Exploring node {}", maze.label());
        let (new_stack, new_labels) = Maze::explore(maze, stack, labels);
        stack = new_stack;
        labels = new_labels;
    }
    (stack, labels)
}

fn maze() -> Rc<Maze> {
    let leaf2 = Maze::new_leaf("2");
    let leaf4 = Maze::new_leaf("4");
    let leaf5 = Maze::new_leaf("5");
    let leaf8 = Maze::new_leaf("8");

    let branch3 = Maze::new_branch("3", leaf4.clone(), leaf5.clone());
    let branch1 = Maze::new_branch("1", leaf2.clone(), branch3.clone());
    let branch7 = Maze::new_branch("7", leaf5.clone(), leaf8.clone());
    let branch6 = Maze::new_branch("6", branch3.clone(), branch7.clone());
    let branch0 = Maze::new_branch("0", branch1.clone(), branch6.clone());

    branch0
}

pub fn main() {
    let root = maze();

    let (_, labels) = loop_explore((vec![root.clone()], vec![]));
    println!("Explored nodes: {:?}", labels);
}
