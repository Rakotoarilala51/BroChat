use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener
};
#[tokio::main]
async fn main() {
    let listner = TcpListener::bind("localhost:8080").await.unwrap();
    loop {
        let (mut socket, _address) = listner.accept().await.unwrap();
        tokio::spawn(async move{
            let (stream_reader, mut stream_writer) = socket.split();
            let mut message = String::new();
            let mut reader = BufReader::new(stream_reader);
            loop {
                let bytes_read = reader.read_line(&mut message).await.unwrap();
                if bytes_read == 0{
                    break;
                }
                stream_writer.write_all(message.as_bytes()).await.unwrap();
                message.clear();
            }
        });

    }
}
