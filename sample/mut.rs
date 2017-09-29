fn main() {
    let x = 1;
    let y: &isize = &x;

    println!("{}", x);
    println!("{}", &x);
    println!("{}", y);
    println!("{}", &y);

    let mut a = 1;
    // let b = &mut a; // <- err
    {
        let b = &mut a;
        *b = 2;
        println!("{}", b);
        println!("{}", *b);
        println!("{}", &b);
    }

    println!("{}", a);
}
