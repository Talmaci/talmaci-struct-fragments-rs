use talmaci_struct_fragments_rs::struct_fragments;

#[struct_fragments]
#[allow(dead_code)]
mod models {
    #[fragment]
    pub struct Identity {
        pub id: i32,
    }

    #[fragment]
    pub struct Timestamps {
        pub created_at: u64,
        pub updated_at: u64,
    }

    pub struct Profile {
        pub bio: String,
    }

    #[compose(Identity, self, Timestamps, profile: Profile)]
    pub struct User {
        pub name: String,
    }
}

fn main() {
    let user = models::User {
        id: 1,
        name: "John".to_owned(),
        created_at: 1,
        updated_at: 2,
        profile: models::Profile {
            bio: "Rust developer".to_owned(),
        },
    };

    println!(
        "{}: {} at {}..{} ({})",
        user.id, user.name, user.created_at, user.updated_at, user.profile.bio
    );
}
