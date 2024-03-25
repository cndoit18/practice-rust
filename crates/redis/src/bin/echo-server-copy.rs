use tokio::io;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6142").await?;

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let (mut rd, mut wr) = io::split(socket);
            if io::copy(&mut rd, &mut wr).await.is_err() {
                eprintln!("failed to copy");
            }
            // 在这里拷贝数据
        });
    }
}
