enum Color {
    Red,
    Blue,
    Green,
    Blank,
}

impl Color {
    fn print(&self) {
        match self {
            Color::Blank => println!("Color: Blank"),
            Color::Blue => println!("Color: Blue"),
            Color::Green => println!("Color: Green"),
            Color::Red => println!("Color: Red"),
        }
    }
}

struct Dimensions {
    height: f64,
    width: f64,
    length: f64,
}

impl Dimensions {
    fn print(&self) {
        println!("Height: {:?}", self.height);
        println!("Width: {:?}", self.width);
        println!("Length: {:?}", self.length);
    }
}
struct ExampleBox {
    dimension: Dimensions,
    weight: f64,
    color: Color,
}

impl ExampleBox {
    fn new(dimension: Dimensions, weight: f64, color: Color) -> Self {
        Self {
            dimension,
            weight,
            color,
        }
    }

    fn print(&self) {
        self.dimension.print();
        println!("Weight: {:?}", self.weight);
        self.color.print();
    }
}

fn main() {
    let my_box = ExampleBox::new(
        Dimensions {
            height: 6.79,
            width: 4.5,
            length: 11.8,
        },
        12.4,
        Color::Green,
    );

    my_box.print();
}
