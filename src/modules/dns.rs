use std::{
    collections::BTreeSet,
    net::IpAddr,
    time::Instant,
};
use anyhow::{Context, bail};
use tokio::net::lookup_host;
use crate::error::Result;

pub async fn lookup(hostname: &str) -> Result<()> {
    let start = Instant::now();
    println!("Starting DNS lookup for {}", hostname);
    let addresses = lookup_host((hostname, 0))
        .await
        .with_context(|| format!("Failed to lookup host '{}'", hostname))?;

    let mut results = BTreeSet::<IpAddr>::new();
    for address in addresses {
        results.insert(address.ip());
    }

    if results.is_empty() {
        bail!("No IP addresses found for host '{}'", hostname);
    }

    let elapsed = start.elapsed();
    println!();
    println!("Successfully resolved!");
    println!("-----------------------");

    let mut ipv4count: i32 = 0;
    let mut ipv6count: i32 = 0;

    for address in &results {
        let version = match address {
            IpAddr::V4(_) => {
                ipv4count += 1;
                "IPv4"
            },
            IpAddr::V6(_) => {
                ipv6count += 1;
                "IPv6"
            },
        };
        println!("{} - {}", address, version);
    }
    println!("-----------------------");
    println!("Matches found: {}", results.len());
    println!("IPv4 count: {}", ipv4count);
    println!("IPv6 count: {}", ipv6count);
    println!("Elapsed time: {:.3?}", elapsed);

    Ok(())
}

