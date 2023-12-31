mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        pub fn seat_at_table() {}
    }

    pub mod serving {
        pub fn take_order() {}
        pub fn serve_order() {}
        pub fn take_payment() {}
    }
}

pub use crate::front_of_house::*;

pub fn eat_at_restaurant() {
    // absolute path
    hosting::add_to_waitlist();
    // relative path
    hosting::add_to_waitlist();
}