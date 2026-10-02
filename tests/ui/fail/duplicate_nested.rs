use talmaci_struct_fragments_rs::struct_fragments;

#[struct_fragments]
mod models {
    #[compose(metadata: u64, metadata: String)]
    struct Model {}
}

fn main() {}

