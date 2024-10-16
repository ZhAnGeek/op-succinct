use anyhow::Result;
use clap::Parser;
use kona_host::{init_tracing_subscriber, start_server, start_server_and_native_client, HostCli};

// Source: https://github.com/ethereum-optimism/kona/blob/main/bin/host/src/main.rs
#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    println!("start");
    let cfg = HostCli::parse();
    init_tracing_subscriber(cfg.v).map_err(|e|  {
        println!("{:?}", e);
        std::process::exit(1);
    });
    println!("end");

    if cfg.server {
        let res = start_server(cfg).await;
        if res.is_err() {
            std::process::exit(1);
        }
    } else {
        let res = start_server_and_native_client(cfg).await;
        if res.is_err() {
            match res {
                Ok(value) => println!("Success: {}", value),
                Err(e) => println!("Error: {}", e),
            }
            std::process::exit(1);
        }
    }

    println!("Exiting host program.");
    std::process::exit(0);
}
