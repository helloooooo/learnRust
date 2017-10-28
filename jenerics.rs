fn pair<T, S>(t: T, s: S) -> (T, S) {
    (t, s)
}

fn main() {
    let i = pair(1, 1.0);
    let i = pair::<isize, f64>(1, 1.0);

    let s = pair("str", "string".to_string());

}
