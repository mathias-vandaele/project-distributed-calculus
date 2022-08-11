use std::net::SocketAddr;
use std::thread;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::Message;
use flume::{Receiver, RecvError, Sender};
use futures_util::{SinkExt, StreamExt};
use tokio::runtime::Runtime;
use tokio_tungstenite::{accept_async, WebSocketStream};
use tokio_tungstenite::tungstenite::protocol::CloseFrame;
use tokio_tungstenite::tungstenite::protocol::frame::Frame;
use crate::counter::Counter;

type Socket = WebSocketStream<TcpStream>;

#[derive(Debug)]
pub enum Event {
    MessageReceived(Connection, String),
    BinaryReceived(Connection, Vec<u8>),
    PingReceived(Connection, Vec<u8>),
    PongReceived(Connection, Vec<u8>),
    CloseReceived(Connection, Option<CloseFrame<'static>>),
    FrameReceived(Connection, Frame),
    ConnectionClosed(Connection),
    ConnectionOpened(Connection),
}


struct ImmatureConnection {
    id: u128,
    event_sender: Sender<Event>,
    tcp_stream: TcpStream,
    socket_addr: SocketAddr,
}

impl ImmatureConnection {
    async fn accept_handshake(self) -> Connection {
        let ws_stream = accept_async(self.tcp_stream)
            .await
            .expect("Error during the websocket handshake occurred");

        println!("WebSocket connection established: {}, Id : {}", self.socket_addr, self.id);

        let connection = Connection {
            id: self.id,
            event_sender: self.event_sender,
            socket: ws_stream,
        };

        connection
    }
}

#[derive(Debug)]
pub struct Connection {
    id: u128,
    event_sender: Sender<Event>,
    socket: Socket,
}


impl Connection {
    fn new(id: u128, event_sender: Sender<Event>, tcp_stream: TcpStream, socket_addr: SocketAddr) -> ImmatureConnection {
        ImmatureConnection { id, event_sender, tcp_stream, socket_addr }
    }

    async fn handle_next_msg(mut self) {
        if let Ok(msg) = self.socket.next().await.expect("Could not read message") {
            match msg {
                Message::Text(message) => {
                    self.event_sender.clone().send(Event::MessageReceived(self, message)).expect("Failed to send `Message Received` notification");
                }
                Message::Binary(binary) => {
                    self.event_sender.clone().send(Event::BinaryReceived(self, binary)).expect("Failed to send `Binary Received` notification");
                }
                Message::Ping(ping) => {
                    self.event_sender.clone().send(Event::PingReceived(self, ping)).expect("Failed to send `Ping Received` notification");
                }
                Message::Pong(pong) => {
                    self.event_sender.clone().send(Event::PongReceived(self, pong)).expect("Failed to send `Pong Received` notification");
                }
                Message::Close(close) => {
                    self.event_sender.clone().send(Event::CloseReceived(self, close)).expect("Failed to send `Close Received` notification");
                }
                Message::Frame(frame) => {
                    self.event_sender.clone().send(Event::FrameReceived(self, frame)).expect("Failed to send `Frame Received` notification");
                }
            }
        } else {
            self.event_sender.clone().send(Event::ConnectionClosed(self)).expect("Failed to send `Connection Closed` example");
        }
    }

    fn send_connection_opened_notification(self) {
        self.event_sender.clone().send(Event::ConnectionOpened(self)).expect("Failed to send `Connection Opened` example");
    }

    pub fn send_message(mut self, message: String) {
        tokio::spawn(async {
            self.socket.send(Message::Text(message)).await.expect("Failed to send message");
            self.handle_next_msg().await;
        });
    }

    pub fn get_id(&self) -> u128 {
        self.id
    }
}

pub struct SocketManager {
    receiver: Receiver<Event>,
}

impl SocketManager {
    pub fn start(addr: String) -> SocketManager {
        let (sender, receiver) = flume::unbounded();
        thread::Builder::new()
            .name("WebSocketManager".to_string())
            .spawn(move || handle_server(addr, sender))
            .expect("Could not start the server");
        SocketManager { receiver }
    }

    pub fn poll_event(&self) -> Result<Event, RecvError> {
        self.receiver.recv()
    }
}

fn handle_server(addr: String, sender: Sender<Event>) {
    Runtime::new().expect("Could not start a new runtime").block_on(async {
        let listener = TcpListener::bind(addr.to_string()).await.expect("Failed to bind socket");
        let mut id_manager = Counter::new(0);
        loop {
            match listener.accept().await {
                Ok((stream, socket_addr)) => {
                    tokio::spawn(handle_new_client(Connection::new(id_manager.next(),
                                                                   sender.clone(),
                                                                   stream,
                                                                   socket_addr)));
                }
                Err(e) => {
                    eprintln!("Impossible to connect with remote client, error : {}", e);
                }
            }
        }
    });
}

async fn handle_new_client(connection: ImmatureConnection) {
    connection
        .accept_handshake().await
        .send_connection_opened_notification();
}
