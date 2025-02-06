struct GroceryItem {
    id: i32,
    quantity: i32
}

fn print_quantity(grocery_item:&GroceryItem) {
    println!("Quantity: {:?}", grocery_item.quantity);
}

fn print_id(grocery_item:&GroceryItem) {
    println!("Id: {:?}", grocery_item.id);
}
fn main() {
    let grocery = GroceryItem {
        id:5,
        quantity: 10
    };

    print_id(&grocery);
    print_quantity(&grocery);
}