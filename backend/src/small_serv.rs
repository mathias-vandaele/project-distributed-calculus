use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
use warp::{cors, Filter};

pub(crate) fn warp(last_prime: Arc<Mutex<u128>>) {
    Runtime::new().expect("warp runtime did not start").block_on(async {
        let cors = cors::cors().allow_any_origin().build();

        let get = warp::path("last-prime")
            .map(move || last_prime.lock().unwrap().to_string())
            .with(cors);

        warp::serve(get)
            .run(([127, 0, 0, 1], 3030))
            .await;
    })
}