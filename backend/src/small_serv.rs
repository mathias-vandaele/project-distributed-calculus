use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
use warp::{cors, Filter};

pub(crate) fn warp(last_prime: Arc<Mutex<u128>>, addr: String) {
    Runtime::new().expect("warp runtime did not start").block_on(async {
        let cors = cors::cors().allow_any_origin().build();

        let get = warp::path("last-prime")
            .map(move || last_prime.lock().unwrap().to_string())
            .with(cors);

        let conf = addr.split(':').collect::<Vec<&str>>();

        let port = conf.get(1).unwrap().parse::<u16>().expect("Not a Valid port");
        let ip = conf.get(0)
            .unwrap()
            .split('.')
            .collect::<Vec<&str>>().iter()
            .map(|&str| str.parse::<u8>().unwrap())
            .collect::<Vec<u8>>();

        let socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(ip[0], ip[1], ip[2], ip[3])), port);

        warp::serve(get)
            .run(socket_addr)
            .await;
    })
}