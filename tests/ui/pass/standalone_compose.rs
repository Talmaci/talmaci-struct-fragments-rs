use talmaci_struct_fragments_rs::compose;

#[derive(Debug)]
struct Timestamps(u64);

#[derive(Debug)]
struct Metadata;

#[compose(
    timestamps: Timestamps,
    metadata: Option<Metadata>,
    tags: Vec<String>,
)]
#[derive(Debug)]
struct User {
    id: i32,
}

#[compose(metadata: Metadata)]
#[derive(Debug)]
struct Wrapper<T>
where
    T: Clone,
{
    value: T,
}

fn main() {
    let user = User {
        id: 1,
        timestamps: Timestamps(2),
        metadata: Some(Metadata),
        tags: vec!["rust".to_owned()],
    };
    let wrapper = Wrapper {
        value: "value",
        metadata: Metadata,
    };

    assert_eq!(user.timestamps.0, 2);
    assert_eq!(user.tags, ["rust"]);
    assert_eq!(wrapper.value, "value");
    let _ = format!("{user:?} {wrapper:?}");
}
