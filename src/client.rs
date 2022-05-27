use capnp_rpc::{rpc_twoparty_capnp, twoparty, RpcSystem};
use futures::{AsyncReadExt, Future, FutureExt};
use log::{info, trace};
use std::pin::Pin;
use std::time::Instant;
use std::{
    net::{SocketAddr, ToSocketAddrs},
    thread,
    time::Duration,
};

use capnp_study::{constant, diamond_capnp, QuxImpl};

pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:4321".to_socket_addrs()?.next().unwrap();

    tokio::task::LocalSet::new().run_until(try_main(addr)).await
}

pub fn print_rose(
    rose_client: diamond_capnp::rose::Client,
) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn std::error::Error>>>>> {
    Box::pin(async move {
        let shape = rose_client.shape_request().send().promise.await?;
        trace!("shape ---");
        thread::sleep(Duration::from_secs(constant::SHORT_SLEEP_SECS));
        let color = rose_client.color_request().send().promise.await?;
        trace!("color ---");
        thread::sleep(Duration::from_secs(constant::SHORT_SLEEP_SECS));
        let name = rose_client.get_name_request().send().promise.await?;
        trace!("name ---");
        thread::sleep(Duration::from_secs(constant::SHORT_SLEEP_SECS));
        let age = rose_client.get_age_request().send().promise.await?;
        trace!("age ---");
        thread::sleep(Duration::from_secs(constant::SHORT_SLEEP_SECS));
        let sub = rose_client.get_sub_request().send().promise.await?;
        trace!("sub ---");
        thread::sleep(Duration::from_secs(constant::SHORT_SLEEP_SECS));
        let s = match shape.get()?.get_s()?.which()? {
            diamond_capnp::rose::shape::Which::Circle(c) => {
                let c = c?.get_radius_request().send().promise.await?;
                trace!("circle radius ---");
                thread::sleep(Duration::from_secs(constant::SHORT_SLEEP_SECS));
                format!("Circle: r = {}", c.get()?.get_r())
            }
            diamond_capnp::rose::shape::Which::Rectangle(r) => {
                let r = r?;
                let w = r.get_width_request().send().promise.await?;
                trace!("rectangle width ---");
                thread::sleep(Duration::from_secs(constant::SHORT_SLEEP_SECS));
                let h = r.get_height_request().send().promise.await?;
                trace!("rectangle height ---");
                thread::sleep(Duration::from_secs(constant::SHORT_SLEEP_SECS));
                format!(
                    "Rectangle: w = {}, h = {}",
                    w.get()?.get_w(),
                    h.get()?.get_h()
                )
            }
        };
        trace!(
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
        trace!("first test");

        let start = Instant::now();

        let mut bar_req = foo.get_bar_request();
        bar_req.get().set_name("Alice".into());
        trace!("NET!");
        thread::sleep(Duration::from_secs(constant::SHORT_SLEEP_SECS));
        let reply = bar_req.send().promise.await?;
        trace!("NET!");
        thread::sleep(Duration::from_secs(constant::SHORT_SLEEP_SECS));
        let bar = reply.get()?.get_bar()?;
        trace!("---");
        thread::sleep(Duration::from_secs(constant::SHORT_SLEEP_SECS));
        let reply = bar.read_val_request().send().promise.await?;
        trace!("NET!");
        thread::sleep(Duration::from_secs(constant::SHORT_SLEEP_SECS));
        let name = reply.get()?.get_val()?.to_string();
        trace!("---");
        thread::sleep(Duration::from_secs(constant::SHORT_SLEEP_SECS));

        let mut baz_req = foo.get_baz_request();
        baz_req.get().set_age(14);
        trace!("---");
        thread::sleep(Duration::from_secs(constant::SHORT_SLEEP_SECS));
        let reply = baz_req.send().promise.await?;
        trace!("NET!");
        thread::sleep(Duration::from_secs(constant::SHORT_SLEEP_SECS));
        let baz = reply.get()?.get_baz()?;
        trace!("---");
        thread::sleep(Duration::from_secs(constant::SHORT_SLEEP_SECS));
        let reply = baz.read_val_request().send().promise.await?;
        trace!("NET!");
        thread::sleep(Duration::from_secs(constant::SHORT_SLEEP_SECS));
        let age = reply.get()?.get_val();
        trace!("---");
        thread::sleep(Duration::from_secs(constant::SHORT_SLEEP_SECS));

        let desc = if age >= 18 { "Adult" } else { "Child" };

        let end = start.elapsed();
        info!(
            "time: {}.{:03}s",
            end.as_secs(),
            end.subsec_nanos() / 1_000_000
        );

        trace!("name: {}({}), age: {}", name, desc, age);
    }

    trace!("wait...");
    thread::sleep(Duration::from_secs(constant::LONG_SLEEP_SECS));
    trace!("done");

    {
        trace!("second test");

        let start = Instant::now();

        let mut bar_req = foo.get_bar_request();
        bar_req.get().set_name("Alice".into());
        trace!("---");
        thread::sleep(Duration::from_secs(constant::SHORT_SLEEP_SECS));
        let bar_client = bar_req.send().pipeline.get_bar();
        trace!("---");
        thread::sleep(Duration::from_secs(constant::SHORT_SLEEP_SECS));

        let mut baz_req = foo.get_baz_request();
        baz_req.get().set_age(14);
        trace!("---");
        thread::sleep(Duration::from_secs(constant::SHORT_SLEEP_SECS));
        let baz_client = baz_req.send().pipeline.get_baz();
        trace!("---");
        thread::sleep(Duration::from_secs(constant::SHORT_SLEEP_SECS));

        let qux_client: diamond_capnp::qux::Client = capnp_rpc::new_client(QuxImpl::new());
        let mut qux_req = qux_client.calc_request();
        trace!("---");
        thread::sleep(Duration::from_secs(constant::SHORT_SLEEP_SECS));
        qux_req.get().set_bar(bar_client);
        trace!("---");
        thread::sleep(Duration::from_secs(constant::SHORT_SLEEP_SECS));
        qux_req.get().set_baz(baz_client);
        trace!("---");
        thread::sleep(Duration::from_secs(constant::SHORT_SLEEP_SECS));
        let reply = qux_req.send().promise.await?;
        trace!("NET!");
        thread::sleep(Duration::from_secs(constant::SHORT_SLEEP_SECS));
        let name = reply.get()?.get_name()?;
        trace!("---");
        thread::sleep(Duration::from_secs(constant::SHORT_SLEEP_SECS));
        let age = reply.get()?.get_age();
        trace!("---");
        thread::sleep(Duration::from_secs(constant::SHORT_SLEEP_SECS));

        let end = start.elapsed();
        info!(
            "time: {}.{:03}s",
            end.as_secs(),
            end.subsec_nanos() / 1_000_000
        );

        trace!("name: {}, age: {}", name, age);
    }

    trace!("wait...");
    thread::sleep(Duration::from_secs(constant::LONG_SLEEP_SECS));
    trace!("done");

    {
        trace!("third test");

        let start = Instant::now();

        let mut counter_request = foo.get_counter_request();
        counter_request.get().set_limit(20);
        let counter_client = counter_request.send().promise.await?.get()?.get_counter()?;

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
            trace!("last c: {}", c);
            thread::sleep(Duration::from_secs(constant::SHORT_SLEEP_SECS));
        }

        let end = start.elapsed();
        info!(
            "time: {}.{:03}s",
            end.as_secs(),
            end.subsec_nanos() / 1_000_000
        );

        trace!("Done");
    }

    trace!("wait...");
    thread::sleep(Duration::from_secs(constant::LONG_SLEEP_SECS));
    trace!("done");

    {
        trace!("fourth test");

        let start = Instant::now();

        let mut counter_request = foo.get_counter_request();
        counter_request.get().set_limit(20);
        let counter_client = counter_request.send().promise.await?.get()?.get_counter()?;

        let c = counter_client
            .run_fast_request()
            .send()
            .promise
            .await?
            .get()?
            .get_count();
        trace!("last c: {}", c);

        let end = start.elapsed();
        info!(
            "time: {}.{:03}s",
            end.as_secs(),
            end.subsec_nanos() / 1_000_000
        );

        trace!("Done");
    }

    trace!("wait...");
    thread::sleep(Duration::from_secs(constant::LONG_SLEEP_SECS));
    trace!("done");

    {
        trace!("fifth test");
        let start = Instant::now();

        let mut counter_request = foo.get_counter_request();
        counter_request.get().set_limit(20);
        let counter_client = counter_request.send().promise.await?.get()?.get_counter()?;

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
            trace!("last c: {}", c);
            thread::sleep(Duration::from_secs(constant::SHORT_SLEEP_SECS));
        }

        let end = start.elapsed();
        info!(
            "time: {}.{:03}s",
            end.as_secs(),
            end.subsec_nanos() / 1_000_000
        );

        trace!("done");
    }

    trace!("wait...");
    thread::sleep(Duration::from_secs(constant::LONG_SLEEP_SECS));
    trace!("done");

    {
        trace!("rose test");

        let start = Instant::now();

        let mut rose_request = foo.get_rose_request();
        rose_request.get().set_depth(3);
        let rose_client = rose_request.send().promise.await?.get()?.get_rose()?;
        print_rose(rose_client).await?;

        let end = start.elapsed();
        info!(
            "time: {}.{:03}s",
            end.as_secs(),
            end.subsec_nanos() / 1_000_000
        );

        trace!("done");
    }

    Ok(())
}
