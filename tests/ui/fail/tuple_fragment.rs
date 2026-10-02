use talmaci_struct_fragments_rs::struct_fragments;

#[struct_fragments]
mod models {
    #[fragment]
    struct Pair(i32, i32);
}

fn main() {}

