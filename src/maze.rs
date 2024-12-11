use Maze::*;

enum Exploration {
    Explored,
    UnExplored,
}

enum Maze<'a> {
    Branch {
        label: String,
        left: &'a Maze<'a>,
        right: &'a Maze<'a>,
        //status: Exploration,  // À ajouter une fois Exploration implémentée
    },
    Leaf {
        label: String,
    },
}

impl<'a> Maze<'a> {
    fn explore(&self, trace: &mut Vec<String>) {
        match self {
            Branch {label, left, right} => {trace.push(label.clone()); left.explore(trace); right.explore(trace); }
            Leaf { label } => {trace.push(label.clone()); }
        }
    }
}


pub fn main() {
    let leaf2 = Maze::Leaf {label: format!("2")};
    let leaf4 = Maze::Leaf {label: format!("4")};
    let leaf5 = Maze::Leaf {label: format!("5")};
    let leaf8 = Maze::Leaf {label: format!("8")};
    let branch3 = Maze::Branch{label: format!("3"), left: &leaf4, right: &leaf5};
    let branch1 = Maze::Branch{label: format!("1"), left: &leaf2, right: &branch3};
    let branch7 = Maze::Branch{label: format!("7"), left: &leaf5, right: &leaf8};
    let branch6 = Maze::Branch{label: format!("6"), left: &branch3, right: &branch7};
    let branch0 = Maze::Branch{label: format!("0"), left: &branch1, right: &branch6};

    let mut trace: Vec<String> = Vec::new();
    branch0.explore(&mut trace);
    println!("{:?}", trace);
}