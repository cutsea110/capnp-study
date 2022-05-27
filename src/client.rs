use capnp_rpc::{pry, rpc_twoparty_capnp, twoparty, RpcSystem};
use futures::{AsyncReadExt, Future, FutureExt};
use log::info;
use std::pin::Pin;
use std::time::Instant;
use std::{
    net::{SocketAddr, ToSocketAddrs},
    thread,
    time::Duration,
};

use capnp_study::{
    diamond_capnp, CounterImpl, NaiveCounterImpl, QuxImpl, RoseImpl, SHORT_SLEEP_SECS,
};

const LONG_SLEEP_SECS: u64 = 0;

pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:3000".to_socket_addrs()?.next().unwrap();

    tokio::task::LocalSet::new().run_until(try_main(addr)).await
}

pub fn print_rose(
    rose_client: diamond_capnp::rose::Client,
) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn std::error::Error>>>>> {
    Box::pin(async move {
        let shape = rose_client.shape_request().send().promise.await?;
        let color = rose_client.color_request().send().promise.await?;
        let name = rose_client.get_name_request().send().promise.await?;
        let age = rose_client.get_age_request().send().promise.await?;
        let sub = rose_client.get_sub_request().send().promise.await?;
        let s = match shape.get()?.get_s()?.which()? {
            diamond_capnp::rose::shape::Which::Circle(c) => {
                format!("Circle")
            }
            diamond_capnp::rose::shape::Which::Rectangle(r) => {
                format!("Rectangle")
            }
        };
        println!(
            "Rose: {}({}) color: {:?}, shape: {}",
            name.get()?.get_name()?,
            age.get()?.get_age(),
            color.get()?.get_color()?,
            s
        );

        for i in 0..sub.get()?.get_sub()?.reborrow().len() {
            let rose = sub.get()?.get_sub()?.reborrow().get(i as u32)?;
            print_rose(rose).await?;
        }

        Ok(())
    })
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

        let start = Instant::now();

        let mut bar_req = foo.get_bar_request();
        bar_req.get().set_name("Alice".into());
        println!("NET!");
        thread::sleep(Duration::from_secs(SHORT_SLEEP_SECS));
        let reply = bar_req.send().promise.await?;
        println!("NET!");
        thread::sleep(Duration::from_secs(SHORT_SLEEP_SECS));
        let bar = reply.get()?.get_bar()?;
        println!("---");
        thread::sleep(Duration::from_secs(SHORT_SLEEP_SECS));
        let reply = bar.read_val_request().send().promise.await?;
        println!("NET!");
        thread::sleep(Duration::from_secs(SHORT_SLEEP_SECS));
        let name = reply.get()?.get_val()?.to_string();
        println!("---");
        thread::sleep(Duration::from_secs(SHORT_SLEEP_SECS));

        let mut baz_req = foo.get_baz_request();
        baz_req.get().set_age(14);
        println!("---");
        thread::sleep(Duration::from_secs(SHORT_SLEEP_SECS));
        let reply = baz_req.send().promise.await?;
        println!("NET!");
        thread::sleep(Duration::from_secs(SHORT_SLEEP_SECS));
        let baz = reply.get()?.get_baz()?;
        println!("---");
        thread::sleep(Duration::from_secs(SHORT_SLEEP_SECS));
        let reply = baz.read_val_request().send().promise.await?;
        println!("NET!");
        thread::sleep(Duration::from_secs(SHORT_SLEEP_SECS));
        let age = reply.get()?.get_val();
        println!("---");
        thread::sleep(Duration::from_secs(SHORT_SLEEP_SECS));

        let desc = if age >= 18 { "Adult" } else { "Child" };

        let end = start.elapsed();
        info!(
            "time: {}.{:03}s",
            end.as_secs(),
            end.subsec_nanos() / 1_000_000
        );

        println!("name: {}({}), age: {}", name, desc, age);
    }

    println!("wait...");
    thread::sleep(Duration::from_secs(LONG_SLEEP_SECS));
    println!("done");

    {
        println!("second test");

        let start = Instant::now();

        let mut bar_req = foo.get_bar_request();
        bar_req.get().set_name("Alice".into());
        println!("---");
        thread::sleep(Duration::from_secs(SHORT_SLEEP_SECS));
        let bar_client = bar_req.send().pipeline.get_bar();
        println!("---");
        thread::sleep(Duration::from_secs(SHORT_SLEEP_SECS));

        let mut baz_req = foo.get_baz_request();
        baz_req.get().set_age(14);
        println!("---");
        thread::sleep(Duration::from_secs(SHORT_SLEEP_SECS));
        let baz_client = baz_req.send().pipeline.get_baz();
        println!("---");
        thread::sleep(Duration::from_secs(SHORT_SLEEP_SECS));

        let qux_client: diamond_capnp::qux::Client = capnp_rpc::new_client(QuxImpl::new());
        let mut qux_req = qux_client.calc_request();
        println!("---");
        thread::sleep(Duration::from_secs(SHORT_SLEEP_SECS));
        qux_req.get().set_bar(bar_client);
        println!("---");
        thread::sleep(Duration::from_secs(SHORT_SLEEP_SECS));
        qux_req.get().set_baz(baz_client);
        println!("---");
        thread::sleep(Duration::from_secs(SHORT_SLEEP_SECS));
        let reply = qux_req.send().promise.await?;
        println!("NET!");
        thread::sleep(Duration::from_secs(SHORT_SLEEP_SECS));
        let name = reply.get()?.get_name()?;
        println!("---");
        thread::sleep(Duration::from_secs(SHORT_SLEEP_SECS));
        let age = reply.get()?.get_age();
        println!("---");
        thread::sleep(Duration::from_secs(SHORT_SLEEP_SECS));

        let end = start.elapsed();
        info!(
            "time: {}.{:03}s",
            end.as_secs(),
            end.subsec_nanos() / 1_000_000
        );

        println!("name: {}, age: {}", name, age);
    }

    println!("wait...");
    thread::sleep(Duration::from_secs(LONG_SLEEP_SECS));
    println!("done");

    {
        println!("third test");

        let start = Instant::now();

        let counter_client: diamond_capnp::counter::Client =
            capnp_rpc::new_client(CounterImpl::new(20));

        while counter_client
            .next_request()
            .send()
            .pipeline
            .get_exist()
            .get_raw_request()
            .send()
            .promise
            .await?
            .get()?
            .get_raw()
        {
            let c = counter_client
                .get_count_request()
                .send()
                .promise
                .await?
                .get()?
                .get_count();
            println!("last c: {}", c);
            thread::sleep(Duration::from_secs(SHORT_SLEEP_SECS));
        }

        let end = start.elapsed();
        info!(
            "time: {}.{:03}s",
            end.as_secs(),
            end.subsec_nanos() / 1_000_000
        );

        println!("Done");
    }

    println!("wait...");
    thread::sleep(Duration::from_secs(LONG_SLEEP_SECS));
    println!("done");

    {
        println!("fourth test");

        let start = Instant::now();

        let counter_client: diamond_capnp::counter::Client =
            capnp_rpc::new_client(CounterImpl::new(20));

        let c = counter_client
            .run_fast_request()
            .send()
            .promise
            .await?
            .get()?
            .get_count();
        println!("last c: {}", c);

        let end = start.elapsed();
        info!(
            "time: {}.{:03}s",
            end.as_secs(),
            end.subsec_nanos() / 1_000_000
        );

        println!("Done");
    }

    println!("wait...");
    thread::sleep(Duration::from_secs(LONG_SLEEP_SECS));
    println!("done");

    {
        println!("fifth test");
        let start = Instant::now();

        let counter_client: diamond_capnp::naive_counter::Client =
            capnp_rpc::new_client(NaiveCounterImpl::new(20));

        while counter_client
            .next_request()
            .send()
            .promise
            .await?
            .get()?
            .get_exist()
        {
            let c = counter_client
                .get_count_request()
                .send()
                .promise
                .await?
                .get()?
                .get_count();
            println!("last c: {}", c);
            thread::sleep(Duration::from_secs(SHORT_SLEEP_SECS));
        }

        let end = start.elapsed();
        info!(
            "time: {}.{:03}s",
            end.as_secs(),
            end.subsec_nanos() / 1_000_000
        );

        println!("done");
    }

    {
        println!("rose test");

        let start = Instant::now();

        let rose_client: diamond_capnp::rose::Client = capnp_rpc::new_client(RoseImpl::new(3));
        print_rose(rose_client).await?;

        let end = start.elapsed();
        info!(
            "time: {}.{:03}s",
            end.as_secs(),
            end.subsec_nanos() / 1_000_000
        );

        println!("done");
    }

    Ok(())
}
