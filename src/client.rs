use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use std::fs::File;
use std::path::PathBuf;

#[tokio::main]
async fn main() {
    let mut socket = TcpStream::connect("127.0.0.1:8080").await.unwrap();
    let file_path = PathBuf::from("path/to/save_large_file");
    let mut file = File::create(file_path).await.unwrap();
    let mut buffer = [0; 1024];

    loop {
        let n = socket.read(&mut buffer).await.unwrap();
        if n == 0 {
            break;
        }
        file.write_all(&buffer[0..n]).await.unwrap();
    }
}
