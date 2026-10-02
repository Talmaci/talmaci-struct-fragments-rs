use talmaci_struct_fragments_rs::struct_fragments;

#[struct_fragments]
#[allow(dead_code)]
mod models {
    #[fragment]
    pub struct UserCore {
        pub id: i32,
        pub name: String,
    }

    #[fragment]
    pub struct Timestamps {
        pub created_at: u64,
        pub updated_at: u64,
    }

    #[compose(UserCore, Timestamps)]
    pub struct InternalUser {
        pub active: bool,
    }

    pub struct Metadata {
        pub label: String,
    }

    #[compose(UserCore, timestamps: Timestamps, metadata: Option<Metadata>)]
    pub struct NestedUser {}
}

#[test]
fn nested_fields_accept_fragment_and_regular_types() {
    let user = models::NestedUser {
        id: 8,
        name: "Grace".to_owned(),
        timestamps: models::Timestamps {
            created_at: 30,
            updated_at: 40,
        },
        metadata: Some(models::Metadata {
            label: "admin".to_owned(),
        }),
    };

    assert_eq!(user.id, 8);
    assert_eq!(user.timestamps.updated_at, 40);
    assert_eq!(user.metadata.expect("metadata should exist").label, "admin");
}

#[test]
fn generated_struct_can_be_constructed_and_accessed() {
    let user = models::InternalUser {
        id: 7,
        name: "Ada".to_owned(),
        created_at: 10,
        updated_at: 20,
        active: true,
    };

    assert_eq!(user.id, 7);
    assert_eq!(user.name, "Ada");
    assert_eq!(user.created_at, 10);
    assert_eq!(user.updated_at, 20);
    assert!(user.active);

    // `repr(C)` would make source field order observable. The UI expansion
    // unit test checks ordering directly without imposing a representation on
    // users' generated structs.
}
