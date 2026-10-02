use talmaci_struct_fragments_rs::compose;

#[derive(Debug)]
struct Timestamps {
    created_at: u64,
    updated_at: u64,
}

#[derive(Debug)]
struct Metadata {
    label: String,
}

#[compose(timestamps: Timestamps, metadata: Option<Metadata>)]
#[derive(Debug)]
struct User {
    id: i32,
    name: String,
}

fn main() {
    let user = User {
        id: 1,
        name: "John".to_owned(),
        timestamps: Timestamps {
            created_at: 1,
            updated_at: 2,
        },
        metadata: Some(Metadata {
            label: "admin".to_owned(),
        }),
    };

    println!(
        "{}: {} at {}..{} ({})",
        user.id,
        user.name,
        user.timestamps.created_at,
        user.timestamps.updated_at,
        user.metadata.as_ref().expect("metadata should exist").label,
    );
}
