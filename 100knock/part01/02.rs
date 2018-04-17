fn main() {
    let result: String = out("パトカー", "タクシー");
    println!("{}", result);
    assert_eq!("パタトクカシーー", result)
}

fn out(_s1: &str, _s2: &str) -> String {
    let mut s1 = _s1.chars();
    let mut s2 = _s2.chars();
    let mut s = String::new();
    for _ in 0..4 {
        s.push(s1.nth(0).unwrap());
        s.push(s2.nth(0).unwrap());
    }
    return s;
}
