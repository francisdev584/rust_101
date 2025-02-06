enum Flavor {
    Sparkling,
    Sweet,
    Fruity,
}

struct Drink {
    flavor: Flavor,
    fluid_oz: f64
}

fn print_drink(drink: Drink) {
    match drink.flavor {
        Flavor::Sparkling => println!("Flavor: Sparkling"),
        Flavor::Fruity => println!("Flavor: Fruity"),
        Flavor::Sweet => println!("Flavor: Sweet")
    }
    println!("Fluid oz: {:?}",drink.fluid_oz);
}
fn main() {
    let my_drink = Drink {
        flavor: Flavor::Sparkling,
        fluid_oz: 5.0,
    };

    print_drink(my_drink);    
}