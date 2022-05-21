use std::net::{SocketAddr, ToSocketAddrs};

use capnp_rpc::{pry, rpc_twoparty_capnp, twoparty, RpcSystem};
use futures::{AsyncReadExt, FutureExt};

use crate::diamond_capnp;

pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:3000".to_socket_addrs()?.next().unwrap();

    tokio::task::LocalSet::new().run_until(try_main(addr)).await
}

async fn try_main(addr: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
    let stream = tokio::net::TcpStream::connect(&addr).await?;
    stream.set_nodelay(true)?;
    let (reader, writer) = tokio_util::compat::TokioAsyncReadCompatExt::compat(stream).split();
    let rpc_network = Box::new(twoparty::VatNetwork::new(
        reader,
        writer,
        rpc_twoparty_capnp::Side::Client,
        Default::default(),
    ));
    let mut rpc_system = RpcSystem::new(rpc_network, None);
    let foo: diamond_capnp::foo::Client = rpc_system.bootstrap(rpc_twoparty_capnp::Side::Server);

    tokio::task::spawn_local(Box::pin(rpc_system.map(|_| ())));

    {
        let mut bar_req = foo.get_bar_request();
        bar_req.get().set_name("Alice".into());
        let name = bar_req
            .send()
            .pipeline
            .get_bar()
            .read_val_request()
            .send()
            .promise
            .await?
            .get()?
            .get_val()?
            .to_string();

        let mut baz_req = foo.get_baz_request();
        baz_req.get().set_age(14);
        let age = baz_req
            .send()
            .pipeline
            .get_baz()
            .read_val_request()
            .send()
            .promise
            .await?
            .get()?
            .get_val();

        println!("name: {}, age: {}", name, age);
    }

    Ok(())
}
