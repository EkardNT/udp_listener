use std::net::{SocketAddr, UdpSocket};
use std::thread::{spawn, JoinHandle};

const FAIL_SOCKET_INIT: i32 = 1;
const FAIL_LOCAL_LOOKUP: i32 = 2;
const FAIL_THREAD_PANIC: i32 = 3;
const FAIL_RECV_FROM: i32 = 4;
const FAIL_BINARY_WRITE: i32 = 5;
const FAIL_TEXT_WRITE: i32 = 6;

fn main() {
    let mut args: Vec<String> = std::env::args().skip(1).collect();

    if args.iter().any(|arg| arg == "help" || arg == "-h" || arg == "--help") {
        show_help();
        return;
    }

    let binary = args
        .iter()
        .position(|arg| arg == "-b" || arg == "--binary")
        .map(|index| args.remove(index))
        .is_some();

    // All remaining args represent socket addresses. First, resolve them all into
    // structured addresses.
    let mut sockets: Vec<UdpSocket> = match args
        .into_iter()
        .map(|text_addr| {
            text_addr
                .parse::<SocketAddr>()
                .map_err(|err| {
                    format!(
                        "Failed to parse argument {} as socket address: {:#?}",
                        text_addr, err
                    )
                })
                .and_then(|socket_addr| {
                    UdpSocket::bind(socket_addr).map_err(|err| {
                        format!(
                            "Failed to bind to UDP address {:?}: {:#?}",
                            socket_addr, err
                        )
                    })
                })
        })
        .collect()
    {
        Ok(sockets) => sockets,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(FAIL_SOCKET_INIT);
        }
    };

    // Print out final bind addr for each socket.
    for socket in &sockets {
        match socket.local_addr() {
            Ok(local_addr) => eprintln!("Bound to {}", local_addr),
            Err(err) => {
                eprintln!("Local addr lookup failed: {:#?}", err);
                std::process::exit(FAIL_LOCAL_LOOKUP);
            }
        }
    }

    // If there's only one, then we can be a bit more efficient by pre-locking the stdout,
    // otherwise we need to use the shared version.
    match sockets.len() {
        1 => {
            let stdout = std::io::stdout();
            let mut stdout = stdout.lock();
            receive_and_forward(binary, sockets.remove(0), &mut stdout);
        },
        _ => {
            let join_handles: Vec<JoinHandle<()>> = sockets
                .into_iter()
                .map(|socket| {
                    spawn(move || {
                        let stdout = std::io::stdout();
                        receive_and_forward(binary, socket, stdout);
                    })
                })
                .collect();
            for handle in join_handles {
                if let Err(err) = handle.join() {
                    eprintln!("Thread panicked: {:#?}", err);
                    std::process::exit(FAIL_THREAD_PANIC);
                }
            }
        }
    }
}

fn receive_and_forward(binary: bool, input: UdpSocket, mut output: impl std::io::Write) {
    let mut buf = [0u8; 1024 * 1024];
    loop {
        let (byte_count, _peer_addr) = match input.recv_from(&mut buf[..]) {
            Ok(data) => data,
            Err(err) => {
                eprintln!("Receive failed: {:#?}", err);
                std::process::exit(FAIL_RECV_FROM);
            }
        };

        // If binary, do a raw write to the output. Otherwise convert to a string, replacing
        // invalid UTF-8 with a replacement char.
        if binary {
            if let Err(err) = output.write_all(&buf[..byte_count]) {
                eprintln!("Binary write to stdout failed: {:#?}", err);
                std::process::exit(FAIL_BINARY_WRITE);
            }
        } else {
            let text = String::from_utf8_lossy(&buf[..byte_count]);
            if let Err(err) = output.write_all(text.as_bytes()) {
                eprintln!("Text write to stdout failed: {:#?}", err);
                std::process::exit(FAIL_TEXT_WRITE);
            }
        }
    }
}

fn show_help() {
    eprintln!("udp_listener [-b|--binary] [<SOCKET_ADDR>]*");
    eprintln!("[Example] Bind to any IPv4 interface on port 12345: udp_listener 0.0.0.0:12345");
    eprintln!("[Example] Bind to any IPv6 address on a random port, in binary mode: udp_listener -b '[::]:0'");
    eprintln!("[Example] Bind to both IPv4 and IPv6 addresses, each with the same port number: udp_listener '0.0.0.0:12345' '[::]:12345'");
}