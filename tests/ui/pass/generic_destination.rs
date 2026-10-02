use talmaci_struct_fragments_rs::struct_fragments;

#[struct_fragments]
mod models {
    #[fragment]
    pub struct Identity {
        pub id: u64,
    }

    #[compose(Identity)]
    pub struct Value<T> {
        pub value: T,
    }
}

fn main() {
    let value = models::Value { id: 3, value: "ok" };
    assert_eq!(value.value, "ok");
}

