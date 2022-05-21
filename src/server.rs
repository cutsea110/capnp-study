use capnp::capability::Promise;
use capnp_rpc::{pry, rpc_twoparty_capnp, twoparty, RpcSystem};
use futures::{AsyncReadExt, FutureExt};
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
        results.get().set_val(self.age);

        Promise::ok(())
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
        params: diamond_capnp::qux::CalcParams,
        mut results: diamond_capnp::qux::CalcResults,
    ) -> Promise<(), capnp::Error> {
        let bar = pry!(pry!(params.get()).get_bar());
        let name: Promise<String, capnp::Error> = Promise::from_future(async move {
            Ok(bar
                .read_val_request()
                .send()
                .promise
                .await?
                .get()?
                .get_val()?
                .to_string())
        });

        let baz = pry!(pry!(params.get()).get_baz());
        let age: Promise<u16, capnp::Error> = Promise::from_future(async move {
            Ok(baz
                .read_val_request()
                .send()
                .promise
                .await?
                .get()?
                .get_val())
        });

        Promise::from_future(async move {
            results.get().set_age(age.await?);
            results.get().set_name(name.await?.as_str());

            Ok(())
        })
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
        let (stream, _) = listener.accept().await?;
        stream.set_nodelay(true)?;
        let (reader, writer) = tokio_util::compat::TokioAsyncReadCompatExt::compat(stream).split();
        let rpc_network = Box::new(twoparty::VatNetwork::new(
            reader,
            writer,
            rpc_twoparty_capnp::Side::Server,
            Default::default(),
        ));

        let rpc_system = RpcSystem::new(rpc_network, Some(foo_client.clone().client));

        tokio::task::spawn_local(Box::pin(rpc_system.map(|_| ())));
    }
}
