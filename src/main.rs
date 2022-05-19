extern crate capnp;
use capnp::capability::Promise;

pub mod diamond_capnp {
    include!(concat!(env!("OUT_DIR"), "/diamond_capnp.rs"));
}

struct FooImpl;
impl diamond_capnp::foo::Server for FooImpl {
    fn get_bar(
        &mut self,
        _: diamond_capnp::foo::GetBarParams,
        _: diamond_capnp::foo::GetBarResults,
    ) -> Promise<(), capnp::Error> {
        panic!("TODO")
    }

    fn get_buz(
        &mut self,
        _: diamond_capnp::foo::GetBuzParams,
        _: diamond_capnp::foo::GetBuzResults,
    ) -> Promise<(), capnp::Error> {
        panic!("TODO")
    }
}

struct BarImpl {
    name: String,
}
impl diamond_capnp::bar::Server for BarImpl {
    fn read_val(
        &mut self,
        _: diamond_capnp::bar::ReadValParams,
        _: diamond_capnp::bar::ReadValResults,
    ) -> Promise<(), capnp::Error> {
        panic!("TODO")
    }
}

struct BuzImpl {
    age: u16,
}
impl diamond_capnp::buz::Server for BuzImpl {
    fn read_val(
        &mut self,
        _: diamond_capnp::buz::ReadValParams,
        _: diamond_capnp::buz::ReadValResults,
    ) -> Promise<(), capnp::Error> {
        panic!("TODO")
    }
}

struct MooImpl;
impl diamond_capnp::moo::Server for MooImpl {
    fn calc(
        &mut self,
        _: diamond_capnp::moo::CalcParams,
        _: diamond_capnp::moo::CalcResults,
    ) -> Promise<(), capnp::Error> {
        panic!("TODO")
    }
}

fn main() {
    println!("Hello")
}
