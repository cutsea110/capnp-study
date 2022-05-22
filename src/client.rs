use capnp_rpc::{rpc_twoparty_capnp, twoparty, RpcSystem};
use futures::{AsyncReadExt, FutureExt};
use std::{
    net::{SocketAddr, ToSocketAddrs},
    thread,
    time::Duration,
};

use capnp_study::{diamond_capnp, CounterImpl, QuxImpl};

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
        println!("first test");

        let mut bar_req = foo.get_bar_request();
        bar_req.get().set_name("Alice".into());
        println!("NET!");
        thread::sleep(Duration::from_secs(1));
        let reply = bar_req.send().promise.await?;
        println!("NET!");
        thread::sleep(Duration::from_secs(1));
        let bar = reply.get()?.get_bar()?;
        println!("---");
        thread::sleep(Duration::from_secs(1));
        let reply = bar.read_val_request().send().promise.await?;
        println!("NET!");
        thread::sleep(Duration::from_secs(1));
        let name = reply.get()?.get_val()?.to_string();
        println!("---");
        thread::sleep(Duration::from_secs(1));

        let mut baz_req = foo.get_baz_request();
        baz_req.get().set_age(14);
        println!("---");
        thread::sleep(Duration::from_secs(1));
        let reply = baz_req.send().promise.await?;
        println!("NET!");
        thread::sleep(Duration::from_secs(1));
        let baz = reply.get()?.get_baz()?;
        println!("---");
        thread::sleep(Duration::from_secs(1));
        let reply = baz.read_val_request().send().promise.await?;
        println!("NET!");
        thread::sleep(Duration::from_secs(1));
        let age = reply.get()?.get_val();
        println!("---");
        thread::sleep(Duration::from_secs(1));

        let desc = if age >= 18 { "Adult" } else { "Child" };

        println!("name: {}({}), age: {}", name, desc, age);
    }

    {
        println!("second test");
        let mut bar_req = foo.get_bar_request();
        bar_req.get().set_name("Alice".into());
        println!("---");
        thread::sleep(Duration::from_secs(1));
        let bar_client = bar_req.send().pipeline.get_bar();
        println!("---");
        thread::sleep(Duration::from_secs(1));

        let mut baz_req = foo.get_baz_request();
        baz_req.get().set_age(14);
        println!("---");
        thread::sleep(Duration::from_secs(1));
        let baz_client = baz_req.send().pipeline.get_baz();
        println!("---");
        thread::sleep(Duration::from_secs(1));

        let qux_client: diamond_capnp::qux::Client = capnp_rpc::new_client(QuxImpl::new());
        let mut qux_req = qux_client.calc_request();
        println!("---");
        thread::sleep(Duration::from_secs(1));
        qux_req.get().set_bar(bar_client);
        println!("---");
        thread::sleep(Duration::from_secs(1));
        qux_req.get().set_baz(baz_client);
        println!("---");
        thread::sleep(Duration::from_secs(1));
        let reply = qux_req.send().promise.await?;
        println!("NET!");
        thread::sleep(Duration::from_secs(1));
        let name = reply.get()?.get_name()?;
        println!("---");
        thread::sleep(Duration::from_secs(1));
        let age = reply.get()?.get_age();
        println!("---");
        thread::sleep(Duration::from_secs(1));

        println!("name: {}, age: {}", name, age);
    }

    {
        println!("third test");

        let counter_client: diamond_capnp::counter::Client =
            capnp_rpc::new_client(CounterImpl::new(20));
        while counter_client
            .next_request()
            .send()
            .promise
            .await?
            .get()?
            .get_exist()
        {
            println!("---");
            let c = counter_client
                .get_count_request()
                .send()
                .promise
                .await?
                .get()?
                .get_count();
            println!("c: {}", c);
        }

        let c = counter_client
            .get_count_request()
            .send()
            .promise
            .await?
            .get()?
            .get_count();
        println!("last c: {}", c);

        println!("Done");
    }

    Ok(())
}
