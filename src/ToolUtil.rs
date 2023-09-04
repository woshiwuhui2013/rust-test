pub struct Info {
    name: String,
    age: i32,
    rating: i32,
}

impl Info {
    pub fn new ()->Info {
       Info {name : "hello".to_string(), age: 40, rating: 30}
    }

    pub fn display(self) {
        println!("name {}, age {} rating {}", self.name, self.age, self.rating)
    }
}