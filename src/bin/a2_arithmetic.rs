fn sum(a: i32, b: i32)-> i32 {
   a + b
}

fn sub(a: i32, b: i32)-> i32 {
    a - b
 }

 fn division(a: i32, b: i32)-> i32 {
    a / b
 }
 fn mult(a: i32, b: i32)-> i32 {
    a * b
 }
 fn remain(a: i32, b: i32)-> i32 {
    a % b
 }
fn main() {
    let result = sum(5, 5);
    println!("{:?}", result);
}