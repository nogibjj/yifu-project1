use ipgeolocate::{Locator, Service};

// print the location of the IP address
// use std::io;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the IP Geolocator!")
}

//create a function that returns the location of an IP address
#[get("/ip/{ip}")]
async fn ip(ip: web::Path<String>) -> impl Responder {
    let service = Service::IpApi;
    let ip = ip.trim();
    // print the location of the IP address
    match Locator::get(ip, service).await {
        Ok(ip) => HttpResponse::Ok().body(format!(
            "{} - {} ({}), {}, ({}, {}), {}",
            ip.ip, ip.city, ip.region, ip.country, ip.latitude, ip.longitude, ip.timezone
        )),
        Err(error) => HttpResponse::Ok().body(format!("Error: {error}")),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello).service(ip))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}

// #[tokio::main]
// async fn main() {
//     let service = Service::IpApi;
//     //input ip address
//     println!("Enter an IP address: ");
//     let mut ip = String::new();
//     io::stdin().read_line(&mut ip).unwrap();
//     let ip = ip.trim();
//     // print the location of the IP address
//     match Locator::get(ip, service).await {
//         Ok(ip) => println!(
//             "{} - {} ({}), {}, ({}, {}), {}",
//             ip.ip, ip.city, ip.region, ip.country, ip.latitude, ip.longitude, ip.timezone
//         ),
//         Err(error) => println!("Error: {error}"),
//     };
// }
