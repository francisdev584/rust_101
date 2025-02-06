fn main() {
    let my_vec = vec![10, 20, 30, 40];

    for item in &my_vec {
        match item {
            30 => println!("thirty"),
            _ => println!("{:?}", item)
        }        
        
        // if item == 30 { // can't compare because it needs borrowed
        //     println!("thirty");
        // } else {
        //     println!("{:?}", item);
        // }
    }
    println!("length: {:?}", my_vec.len());
}
