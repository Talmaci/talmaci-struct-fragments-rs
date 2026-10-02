use talmaci_struct_fragments_rs::struct_fragments;

#[struct_fragments]
mod models {
    #[compose(metadata: u64)]
    struct Model {
        metadata: String,
    }
}

fn main() {}

