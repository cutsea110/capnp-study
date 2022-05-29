use env_logger::Env;
use log::debug;
use structopt::StructOpt;

pub mod client;
pub mod opt;
pub mod server;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("warn")).init();

    let opt = opt::Opt::from_args();
    debug!("Opt: {:?}", opt);

    match opt.program.as_str() {
        "client" => return client::main(&opt).await,
        "server" => return server::main(&opt).await,
        _ => panic!("unknown"),
    }
}
