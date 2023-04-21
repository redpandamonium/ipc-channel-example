mod message;

use clap::Parser;
use ipc_channel::ipc;
use ipc_channel::ipc::{IpcSender};
use crate::message::{IpcExchange, Message};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CmdArgs {
    #[arg(long)]
    ipc_channel: String,
}

fn main() -> anyhow::Result<()> {
    let args = CmdArgs::parse();

    println!("Server: Started.");

    // Establish connection to exchange IPC channels
    let sender = IpcSender::connect(args.ipc_channel)?;

    // Send actual channels to client
    let (c2s_send, c2s_recv) = ipc::channel()?;
    let (s2c_send, s2c_recv) = ipc::channel()?;
    sender.send(IpcExchange {
        sender: c2s_send,
        receiver: s2c_recv,
    })?;

    // Send message to client
    s2c_send.send(Message::ServerToClient { message: "Hello".to_string() })?;

    // Receive message from client
    let msg = c2s_recv.recv()?;
    if let Message::ClientToServer { message } = msg {
        println!("Server: Received {}", message);
    }
    else {
        panic!("Server: Invalid message received {:?}", msg);
    }

    // Done
    Ok(())
}