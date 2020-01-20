use futures::prelude::*;
use tokio::prelude::*;

use tokio_serde::formats::*;
use serde::{Deserialize};
use serde_json::Value;
use tokio::net::{TcpListener, TcpStream};
use tokio_util::codec::{FramedRead, LengthDelimitedCodec};

#[derive(Debug, Deserialize)]
struct Hello {
    from: String,
}

// What is deserialized
type LDFramme = FramedRead<TcpStream, LengthDelimitedCodec>;
type JsonFrame = tokio_serde::SymmetricallyFramed<LDFramme, Hello, SymmetricalJson<Hello>>;

fn create_deserializer(stream: TcpStream) -> JsonFrame {
    let length_delimited = FramedRead::new(stream, LengthDelimitedCodec::new());

    // Deserialize frames
    let mut deserialized = tokio_serde::SymmetricallyFramed::new(
        length_delimited,
        SymmetricalJson::<Hello>::default(),
    );

    return deserialized;
}

#[tokio::main]
pub async fn main() {
    // Bind a server socket
    let mut listener = TcpListener::bind("127.0.0.1:17653").await.unwrap();

    println!("listening on {:?}", listener.local_addr());

    let mut s = listener.incoming();

    while let Some(socket) = s.try_next().await.unwrap() {
        let deserialized = create_deserializer(socket);

        // Spawn a task that prints all received messages to STDOUT
        tokio::spawn(async move {
            while let Some(msg) = deserialized.try_next().await.unwrap() {
                println!("GOT: {:?}", msg);
            }
        });
    }
}
