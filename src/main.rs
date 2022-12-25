use tokio::net::{TcpListener, TcpStream};
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() {
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
    socket.read(&mut buffer).await.unwrap();

    // convert buffer to opcode
    let opcode = RecvOpcode::from_u16(u16::from_le_bytes(buffer)).unwrap();
    println!("Opcode: {:?}", opcode);
}


#[derive(Debug)]
enum RecvOpcode {
    Ping = 1,
}

impl RecvOpcode {
    fn from_u16(value: u16) -> Option<RecvOpcode> {
        match value {
            1 => Some(RecvOpcode::Ping),
            _ => None,
        }
    }
}
