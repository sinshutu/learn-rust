fn main() {

    let a = 1;
    let plus_one = move |x| a + x;

    println!("{}", plus_one(1));
}
