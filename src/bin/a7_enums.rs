enum Color {
    Red,
    Blue,
    Green,
    Yellow
}
fn get_color(color: Color) {
    match color {
      Color::Red => println!("red!"),
      Color::Blue => println!("Blue!"),
      Color::Green => println!("Green! the best one!"),
      Color::Yellow => println!("yellow!")
    }
}
fn main() {
    get_color(Color::Green);
    get_color(Color::Blue);
    get_color(Color::Red);
    get_color(Color::Yellow);
}