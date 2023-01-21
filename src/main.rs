mod radio_server;

fn main()
{
    let server = radio_server::RadioServer::new("127.0.0.1".to_string(), 35058);

    server.listen();
}