mod my_module {
    mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {
                println!("add to waitlist")
            }
        }
    }

    use front_of_house::hosting;
    pub fn eat_at_restaurant() {
        println!("eat at restaurant");
        hosting::add_to_waitlist();
    }
}

use my_module::eat_at_restaurant;
fn main() {
    eat_at_restaurant();
    println!("in main");
    my_module::hosting::add_to_waitlist();
}