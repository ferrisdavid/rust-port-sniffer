use std::{env, process};
use std::io::{self, Write};
use std::net::{IpAddr, TcpStream};
use std::str::FromStr;
use std::sync::mpsc::{Sender, channel};
use std::thread;

// Constant Max Port to Scan. 
const MAX_PORT: u16 = 65535;

// Program Command Line Arguments Struct.
struct Arguments {
    ipaddr: IpAddr,
    threads: u16
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments. you must pass a valid ip address and the number of threads for the process.");
        }
        else if args.len() > 4 {
            return Err("too many arguments.");
        }

        let flag_arg = args[1].clone();
        if let Ok(ipaddr) = IpAddr::from_str(&flag_arg) {
            return Ok(Arguments { ipaddr, threads: 4 });
        }
        else {
            let flag = args[1].clone();
            if flag.contains("-h") || flag.contains("-help") && args.len() == 2 {
                println!("Usage: -j to select how many threads you want -h or -help to show this help message.");
                return  Err("help");
            }
            else if flag.contains("-h") || flag.contains("-help") {
                return Err("too many arguments.")
            }
            else if flag.contains("-j") {
                let ipaddr = match IpAddr::from_str(&args[3]) {
                    Ok(s) => s,
                    Err(_) => return Err("Not a valid IP Address; Must be IPv4 or IPv6.")
                };
                let threads = match args[2].parse::<u16>() {
                    Ok(s) => s,
                    Err(_) => return Err("Failed to parse thread number.")
                };
                return Ok(Arguments { ipaddr, threads });
            }
            else {
                return  Err("invalid usage.");
            }
        }
    }
}

// Port Scan Function - Scans a Range of ports from a specified start port based on the number of active threads. Each thread will call this function to a handle a range of ports concurrently.
fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
    let mut port: u16 = start_port + 1;
    loop {
        match TcpStream::connect((addr, port)) {
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {}
        };

        if (MAX_PORT - port) < num_threads {
            break;
        }
        port += num_threads;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    // Extract and Parse command line args.
    let arguments = Arguments::new(&args).unwrap_or_else(
        |err| {
            if err.contains("help") {
                process::exit(0);
            }
            else {
                eprintln!("{} issue parsing arguments: {}", program, err);
                process::exit(0);
            }
        }
    );

    let num_threads = arguments.threads;
    let (tx, rx) = channel();
    for i in 0..num_threads {
        let tx = tx.clone();

        thread::spawn(move || {
            scan(tx, i, arguments.ipaddr, num_threads);
        });
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
