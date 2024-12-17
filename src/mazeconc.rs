use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug, Clone)]
enum Exploration {
    UnExplored,
    PartiallyExplored,
    Explored,
}

#[derive(Debug)]
enum Maze {
    Branch {
        label: String,
        left: Arc<Maze>,
        right: Arc<Maze>,
        status: Mutex<Exploration>,
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
            status: Mutex::new(Exploration::UnExplored),
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
        work: &Arc<Mutex<Vec<Arc<Maze>>>>,
        trace: &Arc<Mutex<Vec<String>>>,
    ) {
        match self.as_ref() {
            Maze::Branch {
                label,
                left,
                right,
                status,
            } => {
                let mut current_status = status.lock().unwrap();

                match *current_status {
                    Exploration::UnExplored => {

                        *current_status = Exploration::PartiallyExplored;
                        work.lock().unwrap().push(self.clone());
                        work.lock().unwrap().push(left.clone());
                        trace.lock().unwrap().push(label.clone());
                    }
                    Exploration::PartiallyExplored => {

                        *current_status = Exploration::Explored;
                        work.lock().unwrap().push(right.clone());
                        trace.lock().unwrap().push(label.clone());
                    }
                    Exploration::Explored => {
                        // Rien à faire
                    }
                }
            }
            Maze::Leaf { label } => {
                trace.lock().unwrap().push(label.clone());
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
    let work = Arc::new(Mutex::new(vec![maze.clone()])); // Pile partagée
    let trace = Arc::new(Mutex::new(vec![])); // Trace partagée

    let mut handles = vec![];

    for _ in 0..4 {
        let work_clone = Arc::clone(&work);
        let trace_clone = Arc::clone(&trace);

        let handle = thread::spawn(move || {
            while let Some(node) = {
                let mut work_lock = work_clone.lock().unwrap();
                work_lock.pop()
            } {
                node.explore(&work_clone, &trace_clone);
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let final_trace = trace.lock().unwrap();
    println!("Final trace: {:?}", *final_trace);
}
