/// IP address of a server
#[derive(Debug)]
enum IPAddress {
    Local,
    IPv4(u8, u8, u8, u8),
    IPv6(String),
}

/// Server connection defined by an IP Address and a port
struct Server(IPAddress, u64);

fn main() {
    let server1 = Server(IPAddress::IPv4(8,8,8,8), 1234);
    let server2 = Server(IPAddress::Local, 20_000);

    println!("Server 1 address: {:?}", server1.0);
    println!("Server 2 address: {:?}", server2.0);
}
