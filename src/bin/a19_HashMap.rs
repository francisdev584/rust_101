use std::collections::HashMap;
fn main() {
    let mut my_stock = HashMap::new();
    my_stock.insert("Chairs", 5);
    my_stock.insert("Beds", 3);
    my_stock.insert("Tables", 2);
    my_stock.insert("Couches", 0);

    let mut total_stock = 0;
    for (name ,stock) in my_stock.iter() {
        match stock {
            0 => println!("{:?} => Out of stock",name),
            _ => println!("{:?} => {:?}",name, stock)
        }
        total_stock = total_stock + stock;
    }
    println!("total: {:?}", total_stock);

}