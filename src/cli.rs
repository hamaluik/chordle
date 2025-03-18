use clap::{ColorChoice, Parser};
use std::{
    net::{SocketAddr, ToSocketAddrs},
    path::PathBuf,
};

#[derive(Parser, Debug)]
#[command(author = clap::crate_authors!(), version, about, long_about = None, help_template = "\
{before-help}{name} {version}
by {author-with-newline}{about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}
")]
#[command(propagate_version = true)]
pub struct Cli {
    #[arg(short, long, env, default_value_t = ColorChoice::Auto)]
    /// Control whether color is used in the output
    pub colour: ColorChoice,

    /// Enable debugging output
    ///
    /// Use multiple times to increase verbosity (e.g., -v, -vv, -vvv):
    #[arg(short, long, env, action = clap::ArgAction::Count)]
    pub verbose: u8,

    #[arg(short, long, env, default_value = "127.0.0.1:8080", value_parser = parse_socket_addr)]
    /// The address to bind to in the form of <host>:<port>
    ///
    /// To listen on all interfaces, use `0.0.0.0:<port>`
    pub bind: SocketAddr,

    #[arg(short, long, env, default_value = "chordle.db")]
    /// The path to the SQLite database file
    ///
    /// This file will be created if it does not exist
    pub sqlite_db: PathBuf,
}

pub fn cli() -> Cli {
    Cli::parse()
}

fn parse_socket_addr(s: &str) -> Result<SocketAddr, String> {
    s.to_socket_addrs()
        .map_err(|e| e.to_string())?
        .next()
        .ok_or_else(|| format!("{}: no addresses found", s))
}
