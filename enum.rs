enum Command {
    Right(i64),
    Up(i64),
    Move { x: i64, y: i64 },
    Print,
}

fn main() {
    let mut cur = (0, 0);

    let commands = &[
        Command::Move { x: 0, y: 0 },
        Command::Right(5),
        Command::Up(5),
        Command::Print,
        Command::Move { x: 10, y: 10 },
        Command::Print,
    ];
    for c in commands {
        match *c {
            Command::Right(x) => cur.0 += x,
            Command::Up(y) => cur.1 += y,
            Command::Move { x, y } => {
                cur.0 = x;
                cur.1 = y;
            }
            Command::Print => {
                println!("{:?}", cur);
            }
        }
    }
}
