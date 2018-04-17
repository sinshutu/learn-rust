extern crate cli_game;
use cli_game::window;


fn main() {
    let w = window::Window::new();
    let v = w.calc_v();
    println!("{}", v);
}
