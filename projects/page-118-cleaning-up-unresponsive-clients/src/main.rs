extern crate ws;

use std::thread;
use std::time::Duration;
use ws::util;
use ws::{Error, ErrorKind, WebSocket};

const PING: util::Token = util::Token(0);

struct Server {
    out: ws::Sender,
    ping_timeout: Option<util::Timeout>,
}

impl ws::Handler for Server {
    fn on_open(&mut self, _: ws::Handshake) -> ws::Result<()> {
        self.out.timeout(5_000, PING)
    }

    fn on_timeout(&mut self, event: util::Token) -> ws::Result<()> {
        match event {
            PING => {
                println!("Pinging the client");
                self.out.ping("".into())?;
                self.out.timeout(5_000, PING)
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
                self.ping_timeout = Some(timeout);
            }
            _ => {
                eprintln!("Unknown event: {:?}", event);
            }
        }
        Ok(())
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
