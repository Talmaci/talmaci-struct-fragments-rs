use talmaci_struct_fragments_rs::struct_fragments;

#[struct_fragments]
mod models {
    #[fragment]
    pub struct Core {
        pub id: i32,
    }

    #[compose(Core)]
    pub struct Model {
        pub enabled: bool,
    }
}

fn main() {
    let model = models::Model {
        id: 1,
        enabled: true,
    };
    assert!(model.enabled);
}

