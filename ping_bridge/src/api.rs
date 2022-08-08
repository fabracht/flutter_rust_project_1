use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    os::unix::prelude::{AsRawFd, FromRawFd},
    sync::Arc,
    time::Duration,
};

use env_logger::Env;

use log::info;

use pnet_packet::{
    icmp::{self},
    Packet,
};

pub fn run_ping() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // SOURCE IP ADDRESS
    let localhost = Ipv4Addr::UNSPECIFIED;
    let socket_ip_address = SocketAddr::new(IpAddr::V4(localhost), 80);
    let socket2_ip_address = socket_ip_address.into();

    // CREATE ICMP SOCKET
    let socket2_ipv4_socket = socket2::Socket::new(
        socket2::Domain::IPV4,
        socket2::Type::RAW,
        Some(socket2::Protocol::ICMPV4),
    )
    .unwrap();

    // BIND TO LOCAL ADDRESS
    socket2_ipv4_socket
        .bind(&socket2_ip_address)
        .expect(&format!(
            "Failed binding to Ipv4 address {:?}",
            &socket_ip_address
        ));

    // CREATE STD SOCKET FROM SOCKET2 SOCKET
    let raw_ipv4_socket = socket2_ipv4_socket.as_raw_fd();
    let std_ipv4_socket = unsafe { std::net::UdpSocket::from_raw_fd(raw_ipv4_socket) };
    std_ipv4_socket
        .set_read_timeout(Some(Duration::from_millis(100)))
        .unwrap();
    let socket_arc = Arc::new(std_ipv4_socket);
    let dest = "127.0.0.1:0";

    let socket_clone = Arc::clone(&socket_arc);
    std::thread::spawn(move || {
        let packet_slice = &mut [0; 56];
        let mut buf = vec![0; 8 + 56]; // 8 bytes of header, then payload
        let mut packet = icmp::echo_request::MutableEchoRequestPacket::new(&mut buf[..]).unwrap();
        packet.set_icmp_type(icmp::IcmpTypes::EchoRequest);
        packet.set_identifier(1);
        packet.set_sequence_number(1);
        packet.set_payload(packet_slice);

        // Calculate and set the checksum
        let icmp_packet = icmp::IcmpPacket::new(packet.packet()).unwrap();
        let checksum = icmp::checksum(&icmp_packet);
        packet.set_checksum(checksum);
        loop {
            socket_clone.send_to(&mut packet.packet(), dest).unwrap();
            std::thread::sleep(Duration::from_millis(1000));
        }
    });

    let buffer = &mut [0; 1 << 15];
    loop {
        if let Ok((bytes_read, from)) = socket_arc.recv_from(buffer) {
            info!("Received {} bytes from {:?}", bytes_read, from);
            let ipv4_packet = pnet_packet::ipv4::Ipv4Packet::new(&buffer[..bytes_read]).unwrap();
            let _icmp_packet = pnet_packet::icmp::IcmpPacket::new(&ipv4_packet.payload()).unwrap();
            info!("Received {:?}", ipv4_packet);
        }
    }
}

pub fn add_one(number: i32) -> i32 {
    println!("Adding one");
    number + 1
}
