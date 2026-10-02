use talmaci_struct_fragments_rs::struct_fragments;

#[struct_fragments]
mod models {
    #[fragment]
    pub struct A {
        pub first: u8,
    }

    #[fragment]
    pub struct B {
        pub second: u8,
    }

    #[compose(B, A)]
    pub struct Combined {}
}

fn main() {
    let combined = models::Combined {
        second: 2,
        first: 1,
    };
    assert_eq!((combined.second, combined.first), (2, 1));
}

