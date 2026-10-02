use talmaci_struct_fragments_rs::struct_fragments;

#[struct_fragments]
mod models {
    #[fragment]
    pub struct UserCore {
        pub id: i32,
    }

    #[fragment]
    pub struct Timestamps {
        pub created_at: u64,
    }

    pub struct Metadata {
        pub label: &'static str,
    }

    #[compose(UserCore, timestamps: Timestamps, metadata: Option<Metadata>)]
    pub struct User {}
}

fn main() {
    let user = models::User {
        id: 1,
        timestamps: models::Timestamps { created_at: 2 },
        metadata: Some(models::Metadata { label: "public" }),
    };
    assert_eq!(user.timestamps.created_at, 2);
    assert_eq!(user.metadata.unwrap().label, "public");
}

