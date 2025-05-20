use futures_util::{SinkExt, StreamExt};
use http::Uri;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_websockets::{ClientBuilder, Message};

#[tokio::main]
async fn main() -> Result<(), tokio_websockets::Error> {
    let (mut ws_stream, _) =
        ClientBuilder::from_uri(Uri::from_static("ws://127.0.0.1:2000"))
            .connect()
            .await?;

    let stdin = tokio::io::stdin();
    let mut stdin = BufReader::new(stdin).lines();

    loop {
        tokio::select! {
            Ok(Some(line)) = stdin.next_line() => {
                if !line.trim().is_empty() {
                    ws_stream.send(Message::text(line)).await?;
                }
            }
            Some(Ok(msg)) = ws_stream.next() => {
                if let Some(text) = msg.as_text() {
                    println!("{text}");
                }
            }
        }
    }
}