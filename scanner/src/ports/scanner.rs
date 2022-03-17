use super::MOST_COMMON_PORTS;
use crate::types::domain::{Port, Subdomain};
use std::{
    net::{SocketAddr, TcpStream, ToSocketAddrs},
    time::Duration,
};

pub fn scan_common_ports(mut subdomain: Subdomain) -> Subdomain {
    subdomain.open_ports = MOST_COMMON_PORTS
        .iter()
        .map(|port| scan_port(&subdomain.domain, *port))
        .filter(|port| port.is_open)
        .collect();

    subdomain
}

pub fn scan_ports(mut subdomain: Subdomain, ports: Vec<u16>) -> Subdomain {
    subdomain.open_ports = ports
        .iter()
        .map(|port| scan_port(&subdomain.domain, *port))
        .filter(|port| port.is_open)
        .collect();

    subdomain
}

pub fn scan_port(domain: &str, port: u16) -> Port {
    let timeout = Duration::from_secs(3);

    let socket_addresses: Vec<SocketAddr> = format!("{}:{}", domain, port)
        .to_socket_addrs()
        .expect("Failed to covert to socket addresses")
        .collect();

    if socket_addresses.is_empty() {
        return Port {
            port,
            is_open: false,
        };
    }

    Port {
        port,
        is_open: TcpStream::connect_timeout(&socket_addresses[0], timeout).is_ok(),
    }
}
