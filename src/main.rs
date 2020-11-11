fn main() {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    let addr = take_arg_value(&mut args, "-a", "--address").unwrap_or_else(|| "0.0.0.0".to_string());
    let port = take_arg_value(&mut args, "-p", "--port").unwrap_or_else(|| "0".to_string());
    let binary = args.iter().any(|arg| arg == "-b")
        || args.iter().any(|arg| arg == "--binary");

    // Write to standard output.
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();

    // Bind socket.
    let socket = match std::net::UdpSocket::bind(format!("{}:{}", addr, port)) {
        Ok(socket) => socket,
        Err(err) => {
            eprintln!("Bind failed: {:#?}", err);
            return;
        }
    };

    // Print out bind addr. The user might not have provided e.g. a port, in which case the OS
    // would have assigned a random available one.
    match socket.local_addr() {
        Ok(local_addr) => eprintln!("Bound to {}", local_addr),
        Err(err) => {
            eprintln!("Local addr retrieval failed: {:#?}", err);
            return;
        }
    }

    // Read and dump to stdout.
    let mut buf = [0u8; 1024 * 1024];
    loop {
        let (byte_count, _peer_addr) = match socket.recv_from(&mut buf[..]) {
            Ok(data) => data,
            Err(err) => {
                eprintln!("Receive failed: {:#?}", err);
                return;
            }
        };

        // If binary, do a raw write to stdout. Otherwise convert to a string, replacing
        // invalid UTF-8 with a replacement char.
        if binary {
            if let Err(err) = std::io::Write::write_all(&mut stdout, &buf[..byte_count]) {
                eprintln!("Binary write to stdout failed: {:#?}", err);
                return;
            }
        } else {
            let text = String::from_utf8_lossy(&buf[..byte_count]);
            if let Err(err) = std::io::Write::write_all(&mut stdout, text.as_bytes()) {
                eprintln!("Text write to stdout failed: {:#?}", err);
                return;
            }
        }
    }
}

fn take_arg_value<'args>(args: &'args mut Vec<String>, short_arg: &str, long_arg: &str) -> Option<String> {
    args.iter()
        .position(|arg| arg == short_arg || arg == long_arg)
        .map(|arg_index| {
            // Discard the arg name.
            args.remove(arg_index);
            // Return the arg value.
            args.remove(arg_index)
        })
}
