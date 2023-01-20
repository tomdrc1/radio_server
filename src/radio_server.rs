use std::net::TcpListener;

pub struct RadioServer
{
    listener: TcpListener
}


impl RadioServer
{
    pub fn new(ip: String, port: u16) -> RadioServer
    {
        let connection_string: String = format!("{}:{}", ip, port);
        let listener: TcpListener = TcpListener::bind(connection_string).unwrap();

        RadioServer{listener}
    }
}