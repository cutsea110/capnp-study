extern crate capnp;
pub mod friend_capnp {
    include!(concat!(env!("OUT_DIR"), "/friend_capnp.rs"));
}

fn main() {
    println!("Hello")
}
