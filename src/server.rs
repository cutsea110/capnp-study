use capnp::capability::Promise;
use capnp_rpc::{pry, rpc_twoparty_capnp, twoparty, RpcSystem};
use futures::{AsyncReadExt, FutureExt};
use log::trace;
use std::net::{SocketAddr, ToSocketAddrs};

use crate::diamond_capnp;

struct FooImpl;
impl FooImpl {
    pub fn new() -> Self {
        Self
    }
}
impl diamond_capnp::foo::Server for FooImpl {
    fn get_bar(
        &mut self,
        params: diamond_capnp::foo::GetBarParams,
        mut results: diamond_capnp::foo::GetBarResults,
    ) -> Promise<(), capnp::Error> {
        let name = pry!(pry!(params.get()).get_name());
        trace!("get_bar name: {}", name);
        let bar: diamond_capnp::bar::Client = capnp_rpc::new_client(BarImpl::new(name));
        results.get().set_bar(bar);

        Promise::ok(())
    }

    fn get_baz(
        &mut self,
        params: diamond_capnp::foo::GetBazParams,
        mut results: diamond_capnp::foo::GetBazResults,
    ) -> Promise<(), capnp::Error> {
        let age = pry!(params.get()).get_age();
        trace!("get_baz age: {}", age);
        let baz: diamond_capnp::baz::Client = capnp_rpc::new_client(BazImpl::new(age));
        results.get().set_baz(baz);

        Promise::ok(())
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
        mut results: diamond_capnp::bar::ReadValResults,
    ) -> Promise<(), capnp::Error> {
        trace!("get_bar read_val");
        results.get().set_val(self.name.as_str().into());

        Promise::ok(())
    }
}

struct BazImpl {
    age: u16,
}
impl BazImpl {
    pub fn new(age: u16) -> Self {
        Self { age }
    }
}
impl diamond_capnp::baz::Server for BazImpl {
    fn read_val(
        &mut self,
        _: diamond_capnp::baz::ReadValParams,
        mut results: diamond_capnp::baz::ReadValResults,
    ) -> Promise<(), capnp::Error> {
        trace!("get_baz read_val");
        results.get().set_val(self.age);

        Promise::ok(())
    }
}

pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:3000".to_socket_addrs()?.next().unwrap();

    tokio::task::LocalSet::new().run_until(try_main(addr)).await
}

async fn try_main(addr: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    let foo_client: diamond_capnp::foo::Client = capnp_rpc::new_client(FooImpl::new());

    loop {
        trace!("listening...");
        let (stream, _) = listener.accept().await?;
        trace!("accepted");
        stream.set_nodelay(true)?;
        let (reader, writer) = tokio_util::compat::TokioAsyncReadCompatExt::compat(stream).split();
        let rpc_network = Box::new(twoparty::VatNetwork::new(
            reader,
            writer,
            rpc_twoparty_capnp::Side::Server,
            Default::default(),
        ));

        let rpc_system = RpcSystem::new(rpc_network, Some(foo_client.clone().client));

        trace!("spawn");
        tokio::task::spawn_local(Box::pin(rpc_system.map(|_| ())));
    }
}
