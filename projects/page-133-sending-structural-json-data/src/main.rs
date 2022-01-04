extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate ws;

use serde_json::from_str;
use std::time;
use std::time::SystemTime;
use ws::Message;

#[derive(Deserialize, Serialize)]
struct JSONMessage {
    name: String,
    message: String,
}

fn main() {
    ws::listen("127.0.0.1:8080", |out| {
        move |msg: ws::Message| {
            let msg_text = msg.as_text().unwrap();
            if let Ok(json_message) = from_str::<JSONMessage>(msg_text) {
                let now = SystemTime::now()
                    .duration_since(time::UNIX_EPOCH)
                    .expect("Time went backwards");
                let received_at = now.as_millis();
                println!(
                    "{} said: {} at {:?}",
                    json_message.name, json_message.message, received_at
                );
                let output_msg = json!({
                    "name": json_message.name,
                    "message": json_message.message,
                    "received_at": received_at.to_string()
                });

                out.broadcast(Message::Text(output_msg.to_string()))?;
            }
            Ok(())
        }
    })
    .unwrap()
}
