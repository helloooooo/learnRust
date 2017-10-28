trait DuckLike {
    fn quack(&self);

    fn walk(&self) {
        println!("walking");
    }
}

struct Duck;

impl DuckLike for Duck {
    // トレイトで実装されていないメソッドを実装側で定義する
    fn quack(&self) {
        println!("quack");
    }
}
struct Tsuchinoko;

impl DuckLike for Tsuchinoko {
    fn quack(&self) {
        println!("mew");
    }

    fn walk(&self) {
        println!("wriggling");
    }
}

impl DuckLike for i64 {
    fn quack(&self) {
        for _ in 0...&self {
            println!("quack");
        }
    }
}

fn main() {
    let duck = Duck;
    let tuchinoko = Tsuchinoko;
    let i = 3;

    duck.quack();
    tuchinoko.quack();
    i.quack();
}
