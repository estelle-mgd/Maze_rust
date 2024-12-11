use Maze::*;
use Exploration::*;

enum Exploration {
    Explored,
    UnExplored,
}

enum Maze<'a> {
    Branch {
        label: String,
        left: &'a Maze<'a>,
        right: &'a Maze<'a>,
        status: Exploration,
    },
    Leaf {
        label: String,
    },
}

impl<'a> Maze<'a> {

    fn newLeaf(label: String) -> Self {Leaf { label: label } }

    fn newBranch(label: String, left: &'a Maze <'a>, right: &'a Maze <'a>) -> Self {
        Branch { label, left, right, status: UnExplored }
    }

}

/*impl<'a> Maze<'a> {
    fn explore(&self, trace: &mut Vec<String>) {
        match self {
            Branch {label, left, right} => {trace.push(label.clone()); left.explore(trace); right.explore(trace); }
            Leaf { label } => {trace.push(label.clone()); }
        }
    }
}*/

pub fn main() {
    let leaf2 = Maze::Leaf {label: format!("2")};
    let leaf4 = Maze::Leaf {label: format!("4")};
    let leaf5 = Maze::Leaf {label: format!("5")};
    let leaf8 = Maze::Leaf {label: format!("8")};
    let branch3 = Maze::newBranch(format!("3"), &leaf4, &leaf5);
    let branch1 = Maze::newBranch(format!("1"), &leaf2, &branch3);
    let branch7 = Maze::newBranch(format!("7"), &leaf5, &leaf8);
    let branch6 = Maze::newBranch(format!("6"), &branch3, &branch7);
    let branch0 = Maze::newBranch(format!("0"), &branch1, &branch6);
    println!("labyrinthe créé");

    let mut trace: Vec<String> = Vec::new();
    //branch0.explore(&mut trace);
    //println!("{:?}", trace);
}