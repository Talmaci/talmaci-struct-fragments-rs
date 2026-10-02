use talmaci_struct_fragments_rs::compose;

struct Timestamps;

#[compose(timestamps: Timestamps)]
struct User {
    timestamps: String,
}

fn main() {}
