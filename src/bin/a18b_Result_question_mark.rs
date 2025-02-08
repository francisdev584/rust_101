enum ProtectedLocation {
    All,
    Office,
    Warehouse
}
impl ProtectedLocation {
    fn required_access_level(&self) -> u16 {
        match self {
            Self::All => 1000,
            Self::Office => 800,
            Self::Warehouse => 500
        }
    }
}


struct Employee {
    name: String
}

struct KeyCard {
    access_level: u16
}
#[derive(Debug)]

enum AuthorizationStatus {
    Allow,
    Deny
}

#[derive(Debug)]
struct Database;
impl Database {
    fn connect() -> Result<Self,String> {
        Ok(Database)
    }

    fn find_employee(&self, employee_name: &str) -> Result<Employee,String> {
        match employee_name {
            "Anita" => Ok(Employee { name: "Anita".to_string() }),
            "Andressa" => Ok(Employee { name: "Andressa".to_string() }),
            "Catherine" => Ok(Employee { name: "Catherine".to_string() }),
            _ => Err("Usuario não encontrado".to_string())
        }
    }

    fn get_keycard(&self, employee: &Employee) -> Result<KeyCard, String> {
        match employee.name.as_str() {
            "Anita" => Ok(KeyCard {access_level:1000}),
            "Andressa" => Ok(KeyCard { access_level: 500 }),
            "Brody" => Ok(KeyCard { access_level: 800 }),
            other => Err(format!("{other} não tem um keycard"))
        }
    }
} 

fn authorize(
    employee_name: &str,
    location: ProtectedLocation,
) -> Result<AuthorizationStatus,String> {
    // connect to the database
    let db = Database::connect()?;

    // find the employee
    let employee = db.find_employee(employee_name)?;
    
    // get keycard
    let keycard = db.get_keycard(&employee)?;
    
    // determine if the keycard's "access level" is sufficient, using the 
    // 'required_access_level' function implemented on 'ProtectedLocation'.
    let location_access_level = location.required_access_level();
    
    if keycard.access_level >= location_access_level {
        Ok(AuthorizationStatus::Allow)
    } else {
        Ok(AuthorizationStatus::Deny)
    }
}

fn main() {
    let anita_authorized = authorize("Anita", ProtectedLocation::Warehouse);
    println!("{:?}", anita_authorized);
    let andressa_authorized = authorize("Andressa", ProtectedLocation::Office);
    println!("{:?}", andressa_authorized);
    let catherine_authorized = authorize("Catherine", ProtectedLocation::All);
    println!("{:?}", catherine_authorized);
    let brody_authorized = authorize("Brody", ProtectedLocation::Warehouse);
    println!("{:?}", brody_authorized);

}