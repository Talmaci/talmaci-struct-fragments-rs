use talmaci_struct_fragments_rs::struct_fragments;

#[struct_fragments]
mod models {
    #[fragment]
    struct A {
        id: i32,
    }

    #[compose(self, A, self)]
    struct Model {
        local: bool,
    }
}

fn main() {}

