fn print_message(expression: bool) {
    match expression {
        true => println!("its big"),
        false => println!("its small")
    }
}
fn main(){
    let value = 100;
    let is_greater_than_100 = value > 100;
    let is_less_equal_100 = value <= 100; 

    print_message(is_greater_than_100);
    print_message(is_less_equal_100);
}