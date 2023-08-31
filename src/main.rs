use std::{env, process};
use std::net::IpAddr;
use std::str::FromStr;

struct Arguments {
    flag: String,
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
            return Ok(Arguments { flag: String::from(""), ipaddr, threads: 4 });
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
                return Ok(Arguments { flag, ipaddr, threads });
            }
            else {
                return  Err("invalid usage.");
            }
        }
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

    println!("Hello, world!");
}
