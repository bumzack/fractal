use std::fmt::{Debug, Display};

enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

fn main() {
    let dir = Direction::NORTH;

    print_dir(&dir);
    print_dir(&dir);

    let dir2 = &Direction::EAST;
    print_dir(&&&&&dir2);
}

fn print_dir(dir: &Direction) {
    match dir {
        Direction::NORTH => println!("it's north"),
        _ => println!("definitely not NORTH "),
    }
}
