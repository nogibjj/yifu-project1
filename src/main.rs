use ipgeolocate::{Locator, Service};

// print the location of the IP address
use std::io;


#[tokio::main]
async fn main() {
    let service = Service::IpApi;
    //input ip address
    println!("Enter an IP address: ");
    let mut ip = String::new();
    io::stdin().read_line(&mut ip).unwrap();
    let ip = ip.trim();
    // print the location of the IP address
    match Locator::get(ip, service).await {
        Ok(ip) => println!(
            "{} - {} ({}), {}, ({}, {}), {}",
            ip.ip, ip.city, ip.region, ip.country, ip.latitude, ip.longitude, ip.timezone
        ),
        Err(error) => println!("Error: {error}"),
    };
}
