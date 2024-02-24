pub struct Coffee {
    id: String,
    name: String,
    price: u8,
}

impl Coffee {
    pub fn new(id: &str, name: String, price: u8) -> Coffee {
        Coffee {
            id: id.to_string(),
            name,
            price,
        }
    }
    pub fn create_basic_coffees() -> [Coffee; 10] {
        let espresso = Coffee::new("1", "Espresso".to_string(), 20);
        let cappuccino = Coffee::new("2", "Cappuccino".to_string(), 30);
        let latte = Coffee::new("3", "Latte".to_string(), 35);
        let americano = Coffee::new("4", "Americano".to_string(), 25);
        let mocha = Coffee::new("5", "Mocha".to_string(), 40);
        let macchiato = Coffee::new("6", "Macchiato".to_string(), 35);
        let ristretto = Coffee::new("7", "Ristretto".to_string(), 25);
        let affogato = Coffee::new("8", "Affogato".to_string(), 45);
        let flat_white = Coffee::new("9", "Flat White".to_string(), 35);
        let cortado = Coffee::new("10", "Cortado".to_string(), 30);
        [
            espresso, cappuccino, latte, americano, mocha, macchiato, ristretto, affogato,
            flat_white, cortado,
        ]
    }
    pub fn get_formatted_details(&self) -> String {
        format!("{} \t- {} - ${}", self.id, self.name, self.price)
    }
    pub fn get_id(&self) -> &str {
        self.id.as_str()
    }
    pub fn get_price(&self) -> u8 {
        self.price
    }
    pub fn get_price_formatted(&self) -> String {
        let n = format!("${}", self.price);
        n
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
}
