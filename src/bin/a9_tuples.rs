fn print_coord(coord:(i32,i32)) {
    let (_, y) = coord;
     if y > 5 {
        println!("greate than 5");
     } else if y < 5 {
        println!("Less than 5");
     }else {
        println!("Equal 5");
     }
}

fn coordinate() -> (i32,i32) {
    (4,7)
}
fn main() {
    let coord = coordinate();
    print_coord(coord);
}