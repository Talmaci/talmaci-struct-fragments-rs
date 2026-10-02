use talmaci_struct_fragments_rs::compose;

struct Metadata;
struct OtherMetadata;

#[compose(metadata: Metadata, metadata: OtherMetadata)]
struct User {}

fn main() {}
