fn main() {
    println!("{}", out("パタトクカシーー"));
    assert_eq!("パトカー", out("パタトクカシーー"))
}

fn out(s: &str) -> String {
    let mut rtn = String::with_capacity(s.len() / 2);
    for (i, item) in s.chars().enumerate() {
        if i % 2 == 0 {
            rtn.push(item);
        }
    }
    return rtn;
}
