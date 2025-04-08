mod parent {
    pub mod foo {
        pub fn f() {}
    }

    mod bar {
        pub fn g() {}
    }
}

//Think about whether anything outside the parent needs to access that module.
//Other code can access parent::foo::f(), but not parent::bar::g() because bar is private.

mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

// obj = front_of_house::hosting -- This object can use functions, but nothing else can.
// obj = front_of_house -- Cannot use functions
// serving can see hosting and use hosting