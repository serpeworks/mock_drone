
mod mavlink {
    include!(concat!(env!("OUT_DIR"), "/mavlink/mod.rs"));
} use std::time::Duration;

use mavio::{protocol::V2, AsyncReceiver, AsyncSender, Endpoint, Frame, MavLinkId};
pub use mavlink::dialects;
use mavlink::dialects::{serpe_dialect::messages::RegisterAck, SerpeDialect};
use tokio::net::{tcp::OwnedWriteHalf, TcpStream};

const SEND_INTERVAL: Duration = Duration::from_millis(1000);

#[tokio::main]
async fn main() {
    let stream = TcpStream::connect("127.0.0.1:8000").await.unwrap();
    let (reader, writer) = stream.into_split();
    let mut receiver = AsyncReceiver::versioned(reader, V2);
    let mut sender = AsyncSender::versioned(writer, V2);

    send_register_message(&mut sender).await; 
    let system_id = receive_register_ack_message(&mut receiver).await.unwrap(); 
    // send_hearbeats(&mut sender).await;
    // send_unregister_message(&mut sender).await;
    // receiver_unregistered_ack_message(&mut receiver).await;
}

async fn send_register_message(
    sender: &mut AsyncSender<OwnedWriteHalf, V2>,
) {
    let message = dialects::serpe_dialect::messages::Register {
        agent_id: 69,
    };
    let endpoint = Endpoint::v2(MavLinkId::new(0, 0));

    let frame = endpoint.next_frame(&message).unwrap();
    if let Err(error) = sender.send(&frame).await {
        println!("Error sending frame: {:?}", error);
    }

    tokio::time::sleep(SEND_INTERVAL).await;
}

async fn send_hearbeats(
    sender: &mut AsyncSender<OwnedWriteHalf, V2>,
) {
    for _ in 0..10 {
        
        tokio::time::sleep(SEND_INTERVAL).await;
    }
}

async fn send_unregister_message(
    sender: &mut AsyncSender<OwnedWriteHalf, V2>,
) {

}

async fn receive_message
(
    receiver: &mut AsyncReceiver<tokio::net::tcp::OwnedReadHalf, V2>,
) -> Frame<V2> {
    receiver.recv().await.unwrap()
}

async fn receive_register_ack_message(
    receiver: &mut AsyncReceiver<tokio::net::tcp::OwnedReadHalf, V2>,
) -> Result<u8, ()> {
    let frame = receive_message(receiver).await;
    match frame.decode::<SerpeDialect>().unwrap() {
        SerpeDialect::RegisterAck(msg) => {
            let sys_id = msg.system_id;
            Ok(sys_id)
        }
        _ => {
            eprintln!("Unexpected message received: {:?}", frame);
            panic!();
        }         
    } 
}

async fn receiver_unregistered_ack_message(
    receiver: &mut AsyncReceiver<tokio::net::tcp::OwnedReadHalf, V2>,
) {
    let frame = receive_message(receiver).await;
    match frame.decode::<SerpeDialect>().unwrap() {
        SerpeDialect::UnregisterAck(msg) => {
            println!("Received UnregisterAck message: {:?}", msg);
        }
        _ => {
            eprintln!("Unexpected message received: {:?}", frame)
        }         
    }
}
