// Making a module public doesn't make its contents public; it only lets
// its ancestor modules refer to it
pub mod hosting {
    pub fn add_to_waitlist() {}

    fn seat_at_table() {}
}

// hosting and serving are "sibling" modules. Could also move each of these
// into their own files, with front_of_house then being a directory
mod serving {
    fn take_order() {}

    fn serve_order() {}

    fn take_payment() {}
}
