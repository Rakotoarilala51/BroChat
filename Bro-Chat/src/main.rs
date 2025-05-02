use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast
};
#[tokio::main]
async fn main() {
    let listner = TcpListener::bind("localhost:8080").await.unwrap();
    let (tx, _) = broadcast::channel::<String>(10);
    loop {
        let tx = tx.clone();
        let mut rx = tx.subscribe();
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
                tx.send(message.clone()).unwrap();
                let received_message = rx.recv().await.unwrap();
                stream_writer.write_all(received_message.as_bytes()).await.unwrap();
                message.clear();
            }
        });

    }
}
