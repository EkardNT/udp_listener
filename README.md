# udp_listener

A very simple program which listens on a UDP socket address for incoming packets and
writes the received data to standard output. On startup, the program prints the actual
local bind address to standard error. This is useful for when you don't specify a port to
bind to, so that you can see the OS-chosen port.

## Arguments

**`-a <ADDR>`, `--address <ADDR>`**: The IP address to bind to. If not specified, binds to
`[::]` by default. You may specify either an IPv4 or an IPv6 address.

**`-p <PORT>`, `--port <PORT>`**: The UDP port to bind to. If not specified, the OS will
choose a random free port. The local bind address is printed on program startup so that
you can view the randomly chosen port.

**`-b`, `--binary`**: If present, then the received packet data will be written to standard
out unprocessed. If absent, then the received packet data will first be lossily parsed into
UTF-8 text using the U+FFFD REPLACEMENT CHARACTER (�), and only then be written out.