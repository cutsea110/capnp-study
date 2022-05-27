use capnp_rpc::{rpc_twoparty_capnp, twoparty, RpcSystem};
use futures::{AsyncReadExt, FutureExt};
use log::trace;
use std::net::{SocketAddr, ToSocketAddrs};

use capnp_study::{diamond_capnp, FooImpl};

pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:4321".to_socket_addrs()?.next().unwrap();

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
