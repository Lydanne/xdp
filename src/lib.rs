use tokio::net::{ToSocketAddrs, UdpSocket};

type Result<T> = std::result::Result<T, std::io::Error>;

pub struct XDP{
    socket: UdpSocket,
}

impl XDP {
    pub async fn bind<A: ToSocketAddrs>(socket_addrs: A) -> Result<XDP> {
        let socket = UdpSocket::bind(socket_addrs).await?;
        Ok(XDP { socket })
    }

    pub async fn recv_from(&self, buf: &mut [u8]) -> Result<(usize, std::net::SocketAddr)> {
        let (size, addr) = self.socket.recv_from(buf).await?;
        Ok((size, addr))
    }

    pub async fn send_to<A:ToSocketAddrs>(&self, message: &[u8], target: A) -> Result<usize>{
        let size = self.socket.send_to(message, target).await?;
        Ok(size)
    }
}