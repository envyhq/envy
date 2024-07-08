use handler::LspMessageHandler;
use store::FileStore;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, BufReader};
use tokio::net::{UnixListener, UnixStream};
use visitor::LspMessage;

mod handler;
mod store;
mod types;
mod visitor;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    env_logger::init();

    let socket_path = "/tmp/nv_language_server.socket";
    let _ = std::fs::remove_file(socket_path); // Remove the socket file if it exists

    let listener = UnixListener::bind(socket_path)?;
    log::info!(
        "\n\nnv Language Server is listening on unix socket path {}\n\n",
        socket_path
    );

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            if let Err(e) = handle_connection(socket).await {
                log::error!("Error handling connection: {}", e);
            }
        });
    }
}

async fn handle_connection(mut socket: UnixStream) -> Result<(), anyhow::Error> {
    loop {
        let mut reader = BufReader::new(&mut socket);
        let mut content_length_str = String::new();
        let file_store = FileStore::default();

        while let Ok(bytes_read) = reader.read_line(&mut content_length_str).await {
            if bytes_read == 0 || content_length_str.starts_with("Content-Length:") {
                break;
            }
            content_length_str.clear(); // clear if not the content-length line
        }

        let content_length: usize = content_length_str
            .trim_start_matches("Content-Length: ")
            .trim()
            .parse()
            .expect("Failed to parse content length");

        let mut header_newline_separator = [0; 2];
        reader
            .read_exact(&mut header_newline_separator)
            .await
            .expect("Failed to read (skip) header newline separator");

        let mut payload = vec![0; content_length];
        reader
            .read_exact(&mut payload)
            .await
            .expect("Failed to read payload");

        let req = serde_json::from_slice::<LspMessage>(&payload).expect("Failed to deserialize");

        LspMessageHandler
            .handle(&mut socket, req, file_store)
            .await?;
    }
}
