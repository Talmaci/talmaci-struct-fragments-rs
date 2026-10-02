use talmaci_struct_fragments_rs::struct_fragments;

#[struct_fragments]
mod models {
    #[fragment]
    pub struct Identity {
        pub id: i32,
    }

    #[compose(Identity)]
    pub struct User {}
}

fn main() {
    let user = models::User { id: 1 };
    let _ = user.missing;
}
