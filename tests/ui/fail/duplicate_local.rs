use talmaci_struct_fragments_rs::struct_fragments;

#[struct_fragments]
mod models {
    #[fragment]
    struct Core {
        id: i32,
    }

    #[compose(Core)]
    struct Model {
        id: u64,
    }
}

fn main() {}

