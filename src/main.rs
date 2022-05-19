extern crate capnp;
use capnp::capability::Promise;

pub mod diamond_capnp {
    include!(concat!(env!("OUT_DIR"), "/diamond_capnp.rs"));
}

struct FooImpl;
impl FooImpl {
    pub fn new() -> Self {
        Self
    }
}
impl diamond_capnp::foo::Server for FooImpl {
    fn get_bar(
        &mut self,
        _: diamond_capnp::foo::GetBarParams,
        _: diamond_capnp::foo::GetBarResults,
    ) -> Promise<(), capnp::Error> {
        panic!("TODO")
    }

    fn get_baz(
        &mut self,
        _: diamond_capnp::foo::GetBazParams,
        _: diamond_capnp::foo::GetBazResults,
    ) -> Promise<(), capnp::Error> {
        panic!("TODO")
    }
}

struct BarImpl {
    name: String,
}
impl BarImpl {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
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
impl BuzImpl {
    pub fn new(age: u16) -> Self {
        Self { age }
    }
}
impl diamond_capnp::baz::Server for BuzImpl {
    fn read_val(
        &mut self,
        _: diamond_capnp::baz::ReadValParams,
        _: diamond_capnp::baz::ReadValResults,
    ) -> Promise<(), capnp::Error> {
        panic!("TODO")
    }
}

struct QuxImpl;
impl QuxImpl {
    pub fn new() -> Self {
        Self
    }
}
impl diamond_capnp::qux::Server for QuxImpl {
    fn calc(
        &mut self,
        _: diamond_capnp::qux::CalcParams,
        _: diamond_capnp::qux::CalcResults,
    ) -> Promise<(), capnp::Error> {
        panic!("TODO")
    }
}

fn main() {
    println!("Hello")
}
