# udp_listener

A very simple program which listens on a UDP socket address for incoming packets and
writes the received data to standard output. On startup, the program prints the actual
local bind address to standard error. This is useful for when you don't specify a port to
bind to, so that you can see the OS-chosen port.

## Usage

`udp_listener [-b|--binary] [<SOCKET_ADDR>]*`

**`-b`, `--binary`**: If present, then the received packet data will be written to standard
out unprocessed. If absent, then the received packet data will first be lossily parsed into
UTF-8 text using the U+FFFD REPLACEMENT CHARACTER (�), and only then be written out.

**[<SOCKET_ADDR>]\***: Zero or more socket addresses which will be bound to. A socket address
consists of both an IP address and a port number. Both IPv4 and IPv6 are supported. Use port
0 to have the OS assign a random free port.

## Examples

**Example 1**: Bind to any IPv4 interface on the specific port 12345, in text mode.

```udp_listener '0.0.0.0:12345'```

**Example 2**: Bind to any IPv6 address on a random port, in binary mode.

```udp_listener -b '[::]:0'```

**Example 3**: Bind to both an IPv4 and IPv6 address, each with the same port number, in text mode.

```udp_listener '0.0.0.0:12345' '[::]:12345'```