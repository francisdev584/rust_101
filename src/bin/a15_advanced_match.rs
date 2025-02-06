enum Ticket {
    Backstage(String, f64),
    Vip(String, f64),
    Standard(f64),
}

fn main() {
    let my_backstage_ticket = Ticket::Backstage("Francinildo".to_owned(), 100.00);
    let my_vip_ticket = Ticket::Vip("Andressa".to_owned(), 250.00);
    let my_standard_ticket = Ticket::Standard(75.90);

    let tickets = vec![my_backstage_ticket, my_standard_ticket, my_vip_ticket];

    for ticket in tickets {
        match ticket {
            Ticket::Backstage(name, price) => {
                println!("Backstage Ticket. Name: {:?}, Price: {:?}", name, price)
            }
            Ticket::Standard(price) => println!("Ticket Standard. Price: {:?}", price),
            Ticket::Vip(name, price) => {
                println!("Ticket Vip. Name: {:?}, Price: {:?}", name, price)
            }
        }
    }
}
