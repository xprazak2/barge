use clap::Parser;

#[derive(Parser, Debug)]
pub struct Cli {
    /// Port on which the application will run
    #[clap(short, long)]
    pub port: i32,

    /// Bootstrap peer to connect to
    #[clap(short, long)]
    pub bootstrap_peer: Option<i32>,

    /// Interval between heartbeat requests, in sec
    #[clap(short, long, default_value_t = 3)]
    pub interval: u64,
}
