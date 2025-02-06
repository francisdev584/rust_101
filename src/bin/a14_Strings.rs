struct Person {
    age: i32,
    name: String,
    favorite_color: String,
}
// impl Person {
//     fn print(&self) {
//         println!("Name: {:?}", self.name);
//         // println!("Age: {:?}", self.age);
//         println!("Favorite Color: {:?}", self.favorite_color);
//     }
// }

fn print(data: &str) {
    println!("{:?}",data);
}
fn main() {
    let vec_people = vec![
        Person {
            age: 39,
            name: "Francinildo".to_owned(),
            favorite_color: String::from("Green"),
        },
        Person {
            age: 29,
            name: "Andressa".to_owned(),
            favorite_color: String::from("Red"),
        },
        Person {
            age: 9,
            name: "Barbara".to_owned(),
            favorite_color: String::from("Orange"),
        },
    ];
        for person in vec_people {
            if person.age <= 10 {
                print(&person.name);
                print(&person.favorite_color);
            }
        }
    // for person in vec_people {
    //     if person.age <= 10 {
    //         person.print();
    //     }
    // }
}
