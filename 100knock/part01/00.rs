fn main() {
    println!("{}", out("tressed"));
    assert_eq!("desserts", out("stressed"))
}

fn out(s: &str) -> String {
    return s.chars().rev().collect::<String>();
}
