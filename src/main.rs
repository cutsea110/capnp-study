extern crate capnp;

pub mod diamond_capnp {
    include!(concat!(env!("OUT_DIR"), "/diamond_capnp.rs"));
}

pub mod client;
pub mod server;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = ::std::env::args().collect();
    if args.len() >= 2 {
        let sc = args[1].as_str();
        match sc {
            "client" => return client::main().await,
            "server" => return server::main().await,
            _ => panic!("unknown"),
        }
    }

    println!("usage: {} [server | client]", args[0]);
    Ok(())
}
