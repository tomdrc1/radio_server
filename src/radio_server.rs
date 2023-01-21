mod radio_server_handler;

use std::net;
use std::thread;
use std::sync;

pub struct RadioServer
{
    listener: net::TcpListener,
    handler: sync::Arc<radio_server_handler::RadioServerHandler>
}


impl RadioServer
{
    /// Returns a new RadioServer
    /// 
    /// # Arguments
    ///
    /// * `ip` - A string that holds the ip
    /// * `port` - A int that holds the requested port
    /// 
    /// # Example
    /// 
    /// ```
    /// // Set up a server on localhost with port 8080
    /// let radio_server = RadioServer::new("127.0.0.1".to_string(), 8080); 
    /// ```
    pub fn new(ip: String, port: u16) -> RadioServer
    {
        let connection_string: String = format!("{}:{}", ip, port);
        let listener: net::TcpListener = net::TcpListener::bind(connection_string).unwrap();

        RadioServer{listener, handler: sync::Arc::new(radio_server_handler::RadioServerHandler::new())}
    }

    /// Listens for incoming clients forever
    /// 
    /// # Example
    /// 
    /// ```
    /// // Set up a server on localhost with port 8080
    /// let radio_server =  RadioServer::new("127.0.0.1".to_string(), 8080); 
    /// 
    /// // Will now listen forever for incoming clients.
    /// radio_server.listen();
    /// 
    /// ```
    pub fn listen(&self)
    {
        for stream in self.listener.incoming()
        {
            let connection = match stream
            {
                Ok(connection) => connection,
                Err(_) => return
            };
            
            let local_self = self.handler.clone();
            let _ = thread::spawn(move || {
                local_self.handle_client(connection);
            });
        }
    }
}