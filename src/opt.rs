use structopt::{clap, StructOpt};

#[derive(Debug, StructOpt)]
#[structopt(setting(clap::AppSettings::ColoredHelp))]
pub struct Opt {
    #[structopt(short = "r", long = "role", default_value("server"))]
    pub program: String,

    #[structopt(short = "h", long = "host", default_value("127.0.0.1"))]
    pub host: String,

    #[structopt(short = "p", long = "port", default_value("4321"))]
    pub port: u16,
}
