use talmaci_struct_fragments_rs::struct_fragments;

#[struct_fragments]
mod models {
    #[fragment]
    struct Core {
        metadata: String,
    }

    #[compose(Core, metadata: u64)]
    struct Model {}
}

fn main() {}

