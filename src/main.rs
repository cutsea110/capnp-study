use env_logger::Env;

pub mod client;
pub mod server;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("warn")).init();

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
