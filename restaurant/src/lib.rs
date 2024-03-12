mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
pub fn eat_at_restauarant(){

    //Ways of accessing functions within modules.
    //Path 1: Absolute path
    //We define the path starting from the root module i.e. crate
    crate::front_of_house::hosting::add_to_waitlist();

    //Path 2: Relative path
    //We define the path starting from the current module.
    front_of_house::hosting::add_to_waitlist();

}