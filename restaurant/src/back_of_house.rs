// pub before a struct makes it public, but its fields will still be
// private by default. We choose whether to expose each field on a case-by-
// case basis
pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
}

// Because Breakfast has a private field, we need to provide a public
// function to create instances of it; otherwise, we couldn't construct an
// instance of Breakfast because we wouldn't be able to initialize the
// private field
impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches"),
        }
    }
}

// In contrast to the struct case, making an enum public makes all its
// variants public
pub enum Appetizer {
    Soup,
    Salad,
}
