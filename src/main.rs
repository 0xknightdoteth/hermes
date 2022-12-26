use std::error::Error;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

mod recv_opcode;
mod send_opcode;

use recv_opcode::RecvOpcode;
use send_opcode::SendOpcode;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            handle_connection(socket).await;
        });
    }
}

async fn handle_connection(mut socket: TcpStream) {
    let mut buffer = [0; 2];
    let _ = socket.read(&mut buffer).await.unwrap();

    // convert buffer to opcode
    let opcode = RecvOpcode::from_u16(u16::from_le_bytes(buffer)).unwrap();
    println!("Opcode: {:?}", opcode);

    // send opcode back to client
    let opcode = SendOpcode::Pong.to_u16().to_le_bytes();
    socket.write_all(&opcode).await.unwrap();
}
