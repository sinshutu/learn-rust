fn main() {
    let result: String = out("Hi He Lied Because Boron Could Not Oxidize Fluorine.\
                              New Nations Might Also Sign Peace Security Clause.\
                              Arthur King Can.");
    println!("{}", result);
    // "H He Li Be B C N O F Ne Na Mi Al Si P S Cl Ar KCa"
    assert_eq!("HHeLiBeBCNOFNeNaMiAlSiPSClArKCa", result)
}

fn out(s: &str) -> String {
    let mut heads = String::new();
    let one_head_index = [1, 5, 6, 7, 8, 9, 15, 16, 19];
    let ss = s.split_whitespace();
    for item in ss {
        heads.push(item.chars().nth(0).unwrap())
    }
    return heads;
}
