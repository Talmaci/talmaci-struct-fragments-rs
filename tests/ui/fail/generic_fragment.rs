use talmaci_struct_fragments_rs::struct_fragments;

#[struct_fragments]
mod models {
    #[fragment]
    struct Wrapper<T> {
        value: T,
    }
}

fn main() {}

