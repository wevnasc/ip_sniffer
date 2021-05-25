use std::env;
use std::io::{self, Write};
use std::net::{IpAddr, TcpStream};
use std::process;
use std::str::FromStr;
use std::sync::mpsc::{channel, Sender};
use std::thread;

struct Arguments {
    ipaddr: IpAddr,
    threads: u16,
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("not enought arguments");
        }

        if args.len() > 4 {
            return Err("too many arguments");
        }

        let f = args[1].clone();

        if let Ok(ipaddr) = IpAddr::from_str(&f) {
            return Ok(Arguments { ipaddr, threads: 4 });
        } else {
            if f.contains("-h") || f.contains("-help") && args.len() == 2 {
                return Err("help");
            } else if f.contains("-h") || f.contains("-help") {
                return Err("too many arguments");
            } else if f.contains("-j") {
                let ipaddr = match IpAddr::from_str(&args[3]) {
                    Ok(s) => s,
                    Err(_) => return Err("not a valid IPADDR; must be IPV4 or IPV6"),
                };
                let threads = match args[2].parse::<u16>() {
                    Ok(s) => s,
                    Err(_) => return Err("failed to parse threads number"),
                };
                return Ok(Arguments { ipaddr, threads });
            } else {
                return Err("invalid syntaxy");
            }
        }
    }
}

const MAX_PORT: u16 = 65535;

fn scan(sender: Sender<u16>, start_port: u16, addr: IpAddr, threads: u16) {
    let mut port: u16 = start_port + 1;
    loop {
        match TcpStream::connect((addr, port)) {
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                sender.send(port).unwrap();
            }
            Err(_) => {}
        }

        if (MAX_PORT - port) <= threads {
            break;
        }

        port += threads;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let arguments = Arguments::new(&args).unwrap_or_else(|err| {
        if err.contains("help") {
            println!("Usage: -j to selec how many threads you want \r\n -h or -help to show this help message");
            process::exit(0);
        } else {
            eprintln!("{} probem parsing arguments {}", program, err);
            process::exit(0);
        }
    });

    let num_threads = arguments.threads;
    let addr = arguments.ipaddr;
    let (sender, receiver) = channel();
    for i in 0..arguments.threads {
        // it create a sender by thread
        let sender = sender.clone();

        thread::spawn(move || {
            scan(sender, i, addr, num_threads);
        });
    }
    let mut out = vec![];
    drop(sender);

    for value in receiver {
        out.push(value);
    }

    println!("");
    out.sort();
    for value in out {
        println!("{} is open", value);
    }
}
