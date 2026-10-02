use talmaci_struct_fragments_rs::struct_fragments;

#[struct_fragments]
mod models {
    #[fragment]
    pub struct A {
        pub a: u8,
    }

    #[fragment]
    pub struct B {
        pub b: u8,
    }

    #[compose(A, self, B)]
    pub struct Middle {
        pub local: u8,
    }

    #[compose(self, A)]
    pub struct First {
        pub local: u8,
    }

    #[compose(A, self)]
    pub struct Last {
        pub local: u8,
    }

    #[compose(A, self, B)]
    pub struct EmptyLocals {}
}

fn main() {
    let _ = models::Middle { a: 1, local: 2, b: 3 };
    let _ = models::First { local: 1, a: 2 };
    let _ = models::Last { a: 1, local: 2 };
    let _ = models::EmptyLocals { a: 1, b: 2 };
}

