struct Student {
    name: String,
    locker_assignment: Option<i32>
}
fn main() {
    let my_student1 = Student {
        name: "Francis".to_owned(),
        locker_assignment: None
    };

    let my_student2 = Student {
        name: "Andressa".to_owned(),
        locker_assignment: Some(2134)
    };

    println!("Student: {:?}", my_student1.name);
    match my_student1.locker_assignment {
        Some (locker)=> println!("Locker: {:?}",locker),
        None => println!("No locker assigned")
    }

    println!("Student: {:?}", my_student2.name);
    match my_student2.locker_assignment {
        Some (locker)=> println!("Locker: {:?}",locker),
        None => println!("No locker assigned")
    }
}