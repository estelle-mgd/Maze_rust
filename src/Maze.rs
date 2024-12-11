use crate::Maze::Maze::Leaf;

enum Maze {
    Branch {
        label: String,
        left: Box<Maze>,
        right: Box<Maze>,
        //status: Exploration,  // À ajouter une fois Exploration implémentée
    },
    Leaf {
        label: String,
    },
}

fn main() {
    let leaf2 = Leaf(format!("2"));
    let leaf4 = Leaf(format!("4"));
    let leaf5 = Leaf(format!("5"));
    let leaf8 = Leaf(format!("8"));
    let branch3 = Branch(format!("3"), leaf4, leaf5);
    let branch1 = Branch(format!("1"), leaf2, branch3);
    let branch7 = Branch(format!("7"), leaf5, leaf8);
    let branch6 = Branch(format!("6"), branch3, branch7);
    let branch0 = Branch(format!("0"), branch1, branch6);
}