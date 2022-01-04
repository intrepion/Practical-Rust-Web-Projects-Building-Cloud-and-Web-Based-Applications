extern crate ws;

use std::thread;
use std::time::Duration;
use ws::util;
use ws::{CloseCode, Error, ErrorKind, OpCode, WebSocket};

const PING: util::Token = util::Token(0);
const CLIENT_UNRESPONSIVE: util::Token = util::Token(1);

struct Server {
    out: ws::Sender,
    ping_timeout: Option<util::Timeout>,
    client_unresponsive_timeout: Option<util::Timeout>,
}

impl ws::Handler for Server {
    fn on_open(&mut self, _: ws::Handshake) -> ws::Result<()> {
        println!("Opened a connection");
        self.out.timeout(15_000, CLIENT_UNRESPONSIVE)?;
        self.out.timeout(5_000, PING)
    }

    fn on_timeout(&mut self, event: util::Token) -> ws::Result<()> {
        match event {
            PING => {
                println!("Pinging the client");
                self.out.ping("".into())?;
                match self.client_unresponsive_timeout {
                    Some(_) => self.out.timeout(5_000, PING),
                    None => Ok(()),
                }
            }
            CLIENT_UNRESPONSIVE => {
                println!("Client is unresponsive, closing the connection");
                self.client_unresponsive_timeout.take();
                if let Some(timeout) = self.ping_timeout.take() {
                    println!("timeout: {:?}", timeout);
                    self.out.cancel(timeout)?;
                    println!("canceled");
                }
                self.out.close(CloseCode::Away)
            }
            _ => Err(Error::new(
                ErrorKind::Internal,
                "Invalid timeout token encountered!",
            )),
        }
    }

    fn on_new_timeout(&mut self, event: util::Token, timeout: util::Timeout) -> ws::Result<()> {
        match event {
            PING => {
                if let Some(timeout) = self.ping_timeout.take() {
                    self.out.cancel(timeout)?
                }
                match self.client_unresponsive_timeout {
                    Some(_) => {
                        self.ping_timeout = Some(timeout);
                    }
                    None => self.ping_timeout = None,
                }
            }
            CLIENT_UNRESPONSIVE => {
                if let Some(timeout) = self.client_unresponsive_timeout.take() {
                    self.out.cancel(timeout)?
                }
                self.client_unresponsive_timeout = Some(timeout)
            }
            _ => {
                eprintln!("Unknown event: {:?}", event);
            }
        }
        Ok(())
    }

    fn on_frame(&mut self, frame: ws::Frame) -> ws::Result<Option<ws::Frame>> {
        if frame.opcode() == OpCode::Pong {
            println!("Received a pong");
            self.out.timeout(15_000, CLIENT_UNRESPONSIVE)?;
        }

        Ok(Some(frame))
    }

    fn on_close(&mut self, code: ws::CloseCode, reason: &str) {
        println!("Websocket closing for ({:?}) {}", code, reason);
        if let Some(timeout) = self.ping_timeout.take() {
            self.out.cancel(timeout).unwrap()
        }
    }
}

fn main() {
    let server = WebSocket::new(|out| Server {
        out: out,
        ping_timeout: None,
        client_unresponsive_timeout: None,
    })
    .unwrap();

    let broadcaster = server.broadcaster();

    let periodic = thread::spawn(move || loop {
        broadcaster.send("Meow!").unwrap();
        thread::sleep(Duration::from_secs(1));
    });

    server.listen("127.0.0.1:8080").unwrap();

    periodic.join().unwrap();
}
