mod maze;
mod mazerc;
mod mazepile;
mod mazeconc;
mod maze2leaf;

fn main() {
    println!("Hello, world!");
    println!("Maze avec des références TP 3 s3");
    maze::main();
    println!("Maze avec des pointeurs Rc TP 3 s4");
    mazerc::main();
    println!("Maze avec une pile d'exploration TP 3 s5");
    mazepile::main();
    println!("Maze version concurrente TP 4 s1.2");
    mazeconc::main();
    println!("Maze version concurrente avec arrêt à la sortie TP 4 s2");
    maze2leaf::main();
}
