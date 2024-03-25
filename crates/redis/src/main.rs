use mini_redis::{Frame, Connection};
use tokio::net::{TcpListener, TcpStream};


pub const ADDRESS: &'static str = "127.0.0.1:6379";

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind(ADDRESS).await.unwrap();
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
           process(socket).await;
        });
    }
}

async fn process(socket:TcpStream) {
    use std::collections::HashMap;
    use mini_redis::Command::{self, Get, Set};
    let mut db = HashMap::new();
    let mut connection = Connection::new(socket);
    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                // 值被存储为 `Vec<u8>` 的形式
                db.insert(cmd.key().to_string(), cmd.value().to_vec());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                if let Some(value) = db.get(cmd.key()) {
                    // `Frame::Bulk` 期待数据的类型是 `Bytes`， 该类型会在后面章节讲解，
                    // 此时，你只要知道 `&Vec<u8>` 可以使用 `into()` 方法转换成 `Bytes` 类型
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };
        connection.write_frame(&response).await.unwrap();
    }
}