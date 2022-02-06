use clap::Parser;

#[derive(Parser,Debug)]
pub struct Cli {
  #[clap(short, long)]
  pub port: String,

  #[clap(short, long)]
  pub bootstrap_peer: String,
}
