#[derive(Debug)]
struct Adult {
    name: String,
    age: i32
}
impl Adult {
    fn new(name: &str, age: i32) -> Result<Self,&str> {
        if age >= 21 {
            return Ok(Self{name: name.to_owned(), age});
        }else {
            return Err("Erro: precisa ter pelo menos 21 anos");
        }
    }
}

fn main() {
    let my_under_21 = Adult::new("Andressa", 20);
    let my_over_21 = Adult::new("Francinildo", 40);

    println!("{:?}", my_under_21);
    match my_under_21 {
        Ok (value)=> println!("Adult: {:?} / {:?}",value.name, value.age),
        Err (err)=> println!("{:?}",err)
    }

    println!("{:?}", my_over_21);
    match my_over_21 {
        Ok (value)=> println!("Adult: {:?} / {:?}",value.name, value.age),
        Err (err)=> println!("{:?}",err)
    }
}