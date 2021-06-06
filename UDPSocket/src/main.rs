#[warn(non_snake_case)]
use tokio::net::UdpSocket;
use std::error::Error;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let sock = UdpSocket::bind("0.0.0.0:8080").await?;
    let mut buf = [0; 1024];
    dbg!(buf);
    println!("listenning to port 8080\n");
    loop {
        let (len, addr) = sock.recv_from(&mut buf).await?;
        println!("{:?} bytes received from {:?}", len, addr);
        let len = sock.send_to(&buf[..len], addr).await?;
        println!("{:?} bytes sent", len);
    }
}
