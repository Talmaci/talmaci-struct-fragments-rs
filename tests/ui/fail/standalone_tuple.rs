use talmaci_struct_fragments_rs::compose;

struct Metadata;

#[compose(metadata: Metadata)]
struct User(i32);

fn main() {}
