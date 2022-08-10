use futures::{SinkExt, StreamExt};
use gloo_worker::{HandlerId, WorkerScope};
use serde::{Deserialize, Serialize};
use gloo_console::log;
use gloo_net::websocket::futures::WebSocket;
use gloo_net::websocket::Message;
use num::integer::Roots;
use wasm_bindgen_futures::spawn_local;

pub struct Calculator {}

#[derive(Serialize, Deserialize)]
pub struct OutputCalculator {
    id: u128,
    error: String,
}

impl OutputCalculator {
    pub fn get_id(&self) -> u128 {
        self.id
    }

    pub fn get_error_message(self) -> String {
        self.error
    }
}

impl gloo_worker::Worker for Calculator {
    type Message = ();
    type Input = u128;
    type Output = OutputCalculator;

    fn create(_: &WorkerScope<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: &WorkerScope<Self>, _: Self::Message) {
        // Never needed
    }

    fn received(&mut self, scope: &WorkerScope<Self>, msg: Self::Input, id: HandlerId) {
        let scope_clone = scope.clone();
        match WebSocket::open("ws://127.0.0.1:7878") {
            Ok(mut ws) => {
                spawn_local(async move {
                    while let Some(message) = ws.next().await {
                        match message {
                            Ok(Message::Text(message)) => {
                                let nb = message.parse::<u128>().unwrap();
                                let result = is_prime(nb);
                                ws.send(Message::Text(result.to_string())).await.expect("Could not send the message");
                            }
                            Ok(Message::Bytes(_)) => {
                                // DO NOTHING
                            }
                            Err(e) => {
                                scope_clone.respond(id, OutputCalculator { id: msg, error: format!("An Error has been detected trying to receive a message {}", e.to_string()) });
                                break;
                            }
                        }
                    }
                });
            }
            Err(e) => {
                scope_clone.respond(id, OutputCalculator { id: msg, error: format!("An error happened trying to create the connection : {}", e.to_string()) });
            }
        };
    }

    fn disconnected(&mut self, _: &WorkerScope<Self>, _: HandlerId) {
        log!("One worker is dropped");
    }
}

fn is_prime(n: u128) -> bool {
    let squared_root = n.sqrt() + 1;
    for i in 2..squared_root {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}