use std::io::{self, Write};
use std::sync::mpsc::{Sender, channel};

// Tokio + Network
use std::net::{IpAddr, Ipv4Addr};
use tokio::task;
use tokio::net::TcpStream;

// CLI
use bpaf::Bpaf;

// Constant Max Port to Scan. 
const MAX_PORT: u16 = 65535;

// Fallback IP Address.
const IPFALLBACK: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

// Program Command Line Arguments Struct.
#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
pub struct Arguments {
    #[bpaf(long, short, fallback(IPFALLBACK))]
    /// The address that you want to sniff.  Must be a valid ipv4 address.  Falls back to 127.0.0.1
    pub address: IpAddr,
    #[bpaf(long("start"), short('s'), guard(start_port_guard, "Must be greater than 0."), fallback(1 as u16))]
    /// The start port for the sniffer. (must be greater than 0)
    pub start_port: u16,
    #[bpaf(long("end"), short('e'), guard(end_port_guard, "Must be less than or equal to 65535."), fallback(MAX_PORT))]
    /// The end port for the sniffer. (must be less than or equal to 65535)
    pub end_port: u16
}

// Command Line Argument Guards.
fn start_port_guard(input: &u16) -> bool {
    return *input > 0;
}
fn end_port_guard(input: &u16) -> bool {
    return *input <= MAX_PORT;
}

// Port Scan Function - Scans a port to determine whether it is open for connection.
async fn scan(tx: Sender<u16>, port: u16, addr: IpAddr) {
    match TcpStream::connect(format!("{}:{}", addr, port)).await {
        Ok(_) => {
            print!(".");
            io::stdout().flush().unwrap();
            tx.send(port).unwrap();
        }
        Err(_) => {}
    }
}

#[tokio::main]
async fn main() {
    // Collect the command line arguments
    let args = arguments().run();
    // Initialize the sender and receiver channel.
    let (tx, rx) = channel();
    // Loop through range of ports and spawn async scan tasks to check for open ports.
    for i in args.start_port..args.end_port {
        let tx = tx.clone();
        task::spawn(async move { scan(tx, i, args.address).await });
    }

    // Collect the ports sent to receiver into output vector.
    let mut out = vec![];
    drop(tx);
    for p in rx {
        out.push(p);
    }

    // Print Open Ports
    println!("");
    out.sort();
    for port in out {
        println!("{} is open", port);
    }
}
