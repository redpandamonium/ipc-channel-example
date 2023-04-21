mod message;

use std::io::stdout;
use std::process::{Command, Stdio};
use clap::Parser;
use ipc_channel::ipc::IpcOneShotServer;
use crate::message::{IpcExchange, Message};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CmdArgs {
    #[arg(long)]
    server_binary: String,
}

fn main() -> anyhow::Result<()> {
    let args = CmdArgs::parse();

    println!("Client: Started client.");

    // Start ipc server
    let (ipc, ipc_channel_name) = IpcOneShotServer::<IpcExchange>::new()?;

    // Start child process, letting it know the server file path
    let handle = Command::new(args.server_binary.as_str())
        .arg("--ipc-channel")
        .arg(ipc_channel_name)
        .stdout(Stdio::inherit()) // child process prints to our stdout
        .spawn()?;

    println!("Client: Started Server.");

    // Establish ipc connection
    let (_, IpcExchange { sender, receiver }) = ipc.accept()?;

    // Receive message from server
    let msg = receiver.recv()?;
    if let Message::ServerToClient { message } = msg {
        println!("Client: Received message {}", message);
    }
    else {
        panic!("Client: Received invalid message {:?}", msg);
    }

    // Send message back to the server
    sender.send(Message::ClientToServer { message: "World".to_string() })?;

    // Wait for server process to exit
    let output = handle.wait_with_output()?;
    assert_eq!(output.status.code().unwrap_or(-1), 0);

    // Done
    Ok(())
}