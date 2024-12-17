use std::cell::RefCell;
use std::sync::Arc;

#[derive(Debug, Clone)]
enum Exploration {
    UnExplored,
    PartiallyExplored,
    Explored,
}

#[derive(Debug, Clone)]
enum Maze {
    Branch {
        label: String,
        left: Arc<Maze>,
        right: Arc<Maze>,
        status: RefCell<Exploration>,
    },
    Leaf {
        label: String,
    },
}

impl Maze {
    fn new_branch(label: &str, left: Arc<Maze>, right: Arc<Maze>) -> Arc<Maze> {
        Arc::new(Maze::Branch {
            label: label.to_string(),
            left,
            right,
            status: RefCell::new(Exploration::UnExplored),
        })
    }

    fn new_leaf(label: &str) -> Arc<Maze> {
        Arc::new(Maze::Leaf {
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
        self: &Arc<Maze>,
        work: &mut Vec<Arc<Maze>>,
        trace: &mut Vec<String>,
    ) {
        match &**self {
            Maze::Branch {
                label,
                left,
                right,
                status,
            } => {
                let mut current_status = status.borrow_mut();
                match *current_status {
                    Exploration::UnExplored => {
                        // Passer à PartiallyExplored et empiler la branche
                        *current_status = Exploration::PartiallyExplored;
                        work.push(self.clone());
                        work.push(left.clone());
                        trace.push(label.clone());
                    }
                    Exploration::PartiallyExplored => {
                        // Passer à Explored et explorer la branche droite
                        *current_status = Exploration::Explored;
                        work.push(right.clone());
                        trace.push(label.clone());
                    }
                    Exploration::Explored => {
                        // Rien à faire si déjà exploré
                    }
                }
            }
            Maze::Leaf { label } => {
                // Ajouter la feuille à la trace
                trace.push(label.clone());
            }
        }
    }
}

fn maze() -> Arc<Maze> {
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
    let maze = maze();
    let mut work = vec![maze.clone()];
    let mut trace = vec![];

    while !work.is_empty() {
        let node = work.pop().expect("work stack should not be empty");
        node.explore(&mut work, &mut trace);
        println!("trace so far: {:?}", trace);
    }

    println!("Final trace: {:?}", trace);
}
