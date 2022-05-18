use capnp::message::{Builder, HeapAllocator};

extern crate capnp;
pub mod friend_capnp {
    include!(concat!(env!("OUT_DIR"), "/friend_capnp.rs"));
}
use friend_capnp::person;

fn person_new(name: &str, age: u16) -> Builder<HeapAllocator> {
    let mut builder = capnp::message::Builder::new_default();
    {
        let mut p = builder.init_root::<person::Builder>();
        p.set_name(name.into());
        p.set_age(age);
    }
    builder
}

fn main() {
    let alice = person_new("Alice", 27);
    let bob = person_new("Bob", 42);
    let charlie = person_new("Charlie", 51);
}
