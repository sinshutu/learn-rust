fn main() {
    let result = out("Now I need a drink, alcoholic of course,\
                             after the heavy lectures involving quantum mechanics.");
    println!("{}", result[0]);
    println!("asdf");
    assert_eq!(["I",
                "Now",
                "a",
                "after",
                "alcoholic",
                "course",
                "drink",
                "heavy",
                "involving",
                "lectures",
                "mechanics",
                "need",
                "of",
                "quantum",
                "the"].to_vec(),
               result)
}

fn out(s: &str) -> Vec<String> {
    let ss: Vec<String> = s.split_whitespace().collect<Vec<&str>>().map(|i| i.to_string());
    for item in ss.into_iter() {
        // println!("{}", item.replace(",", ""));
        item = item.replace(",", "");
        item = item.replace(".", "");
        println!("{}", item);
    }
    return ss
}
