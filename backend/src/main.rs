extern crate core;

mod socket_manager;
mod counter;
mod small_serv;
mod config;

use std::collections::{HashMap, VecDeque};
use std::process::exit;
use std::sync::{Arc, Mutex};
use std::{env, process, thread};
use crate::config::Config;
use crate::counter::Counter;
use crate::socket_manager::{Event};

#[tokio::main]
async fn main() {

    let args : Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else( |err| {
        eprintln!("Problem parsing arguments : {}", err);
        process::exit(1);
    });

    let mut socket_addr = config.ip.clone();
    socket_addr.push_str(":7878");

    let mut server_addr = config.ip.clone();
    server_addr.push_str(":3030");

    println!("socket server starting on : {}", socket_addr);
    println!("http server starting on : {}", server_addr);

    let sm = socket_manager::SocketManager::start(socket_addr);
    let mut id_worker_number_map: HashMap<u128, u128> = HashMap::new();
    let mut job: VecDeque<u128> = (0..10000).collect::<VecDeque<u128>>();
    let mut number_manager = Counter::new(10000);
    let last_prime = Arc::new(Mutex::new(1));


    let warped = last_prime.clone();
    thread::spawn(move || small_serv::warp(warped, server_addr));

    loop {
        let event = match sm.poll_event() {
            Ok(event) => { event }
            Err(e) => {
                eprintln!("An unexpected error happened whe trying to poll an event : {}", e);
                exit(1);
            }
        };

        match event {
            Event::ConnectionOpened(connection) => {
                let next = job.pop_front().unwrap();
                id_worker_number_map.entry(connection.get_id()).or_insert(next);
                connection.send_message(next.to_string());
                job.push_back(number_manager.next())
            }
            Event::MessageReceived(connection, message) => {
                let value = id_worker_number_map.get_mut(&connection.get_id()).expect("An unexpected error happened");
                if message == "true" {
                    //println!("worker n°{} has found a prime number : {}", connection.get_id(), value);
                    *last_prime.lock().unwrap() = *value
                }
                let next = job.pop_front().unwrap();
                connection.send_message(next.to_string());
                *value = next;
                job.push_back(number_manager.next());
            }
            Event::ConnectionClosed(connection) => {
                let value = id_worker_number_map.get(&connection.get_id()).expect("An unexpected error happened");
                println!("Worker n°{} has closed the connection without finishing his job, his number {} goes back to the queue", connection.get_id(), value);
                job.push_back(*value);
                id_worker_number_map.remove(&connection.get_id()).expect("This worker should have been in the map");
            }
            Event::CloseReceived(connection, _close_frame) => {
                let value = id_worker_number_map.get(&connection.get_id()).expect("An unexpected error happened");
                println!("Worker n°{} has closed the connection without finishing his job, his number {} goes back to the queue", connection.get_id(), value);
                job.push_back(*value);
                id_worker_number_map.remove(&connection.get_id()).expect("This worker should have been in the map");
            }
            e => println!("{:?}", e),
        }
    }
}
