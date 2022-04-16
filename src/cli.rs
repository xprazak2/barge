use clap::Parser;

#[derive(Parser,Debug)]
pub struct Cli {
  #[clap(short, long)]
  pub port: i32,

  #[clap(short, long)]
  pub bootstrap_peer: Option<i32>,
}
