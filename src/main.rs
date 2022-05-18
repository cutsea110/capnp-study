extern crate capnp;
use capnp::capability::Promise;

pub mod friend_capnp {
    include!(concat!(env!("OUT_DIR"), "/friend_capnp.rs"));
}
use friend_capnp::person;

struct PersonImpl;
impl person::Server for PersonImpl {
    fn get_age(
        &mut self,
        _: person::GetAgeParams,
        _: person::GetAgeResults,
    ) -> Promise<(), capnp::Error> {
        panic!("TODO")
    }
    fn get_name(
        &mut self,
        _: person::GetNameParams,
        _: person::GetNameResults,
    ) -> Promise<(), capnp::Error> {
        panic!("TODO")
    }
}

fn main() {
    let client: friend_capnp::person::Client = capnp_rpc::new_client(PersonImpl);
    println!("Hello")
}
