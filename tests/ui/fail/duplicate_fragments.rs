use talmaci_struct_fragments_rs::struct_fragments;

#[struct_fragments]
mod models {
    #[fragment]
    struct A {
        id: i32,
    }

    #[fragment]
    struct B {
        id: u64,
    }

    #[compose(A, B)]
    struct Model {}
}

fn main() {}

