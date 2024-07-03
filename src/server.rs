use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

async fn handle_client(mut socket: TcpStream, file_path: PathBuf) {
    let file = File::open(file_path).await.unwrap();
    let mut reader = BufReader::new(file);
    let mut buffer = [0; 1024];

    loop {
        let n = reader.read(&mut buffer).await.unwrap();
        if n == 0 {
            break;
        }
        socket.write_all(&buffer[0..n]).await.unwrap();
    }
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    let file_path = PathBuf::from("path/to/large_file");

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let file_path = file_path.clone();
        tokio::spawn(async move {
            handle_client(socket, file_path).await;
        });
    }
}
