use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use bytes::Bytes;
use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

pub const ADDRESS: &'static str = "127.0.0.1:6379";

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind(ADDRESS).await.unwrap();
    println!("Listening");
    let db = Arc::new(Mutex::new(HashMap::<String, Bytes>::new()));
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let db = db.clone();
        println!("Accepted");
        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

async fn process(socket: TcpStream, db: Db) {
    use mini_redis::Command::{self, Get, Set};
    let mut connection = Connection::new(socket);
    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                // 值被存储为 `Vec<u8>` 的形式
                db.lock()
                    .unwrap()
                    .insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                if let Some(value) = db.lock().unwrap().get(cmd.key()) {
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
