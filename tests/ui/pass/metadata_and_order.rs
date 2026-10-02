use talmaci_struct_fragments_rs::struct_fragments;

#[struct_fragments]
mod models {
    #[fragment]
    pub struct Metadata {
        /// This documentation is cloned with the complete field AST.
        pub documented: u8,

        #[cfg(any())]
        pub removed_by_copied_attribute: u8,
    }

    #[compose(Metadata)]
    pub struct Model {
        pub local: u8,
    }
}

fn main() {
    // Public visibility and the copied `cfg` attribute are both observable here.
    let model = models::Model {
        documented: 1,
        local: 2,
    };
    assert_eq!((model.documented, model.local), (1, 2));
}

