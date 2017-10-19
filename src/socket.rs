// use std::os::unix::net::UnixStream;
use std::os::unix::net::{UnixStream, UnixListener};
use std::io::{Write, Read};

pub struct server
{
    socket: UnixListener,
}

impl server 
{
    pub fn new(socketAddress: String) -> server
    {
        let socket = match UnixListener::bind(socketAddress)
        {
            Ok(sock) => sock,
            Err(e) => 
            {
                panic!("Couldn't connect: {:?}", e);
            }
        };
        server{socket:socket}
    }

    fn accept(&self) -> clientSocket
    {
        match self.socket.accept() 
        {
            Ok((clientStream, clientAddr)) => return clientSocket{stream: clientStream},
            Err(e) => panic!("accept function failed.. {}", e),
        };
    }
}

struct clientSocket
{
    stream: UnixStream,
}

impl clientSocket
{

    fn send(&mut self, output: String)
    {
        match self.stream.write_all(output.as_bytes())
        {
            Ok(nothing) => {},
            Err(e) => panic!("writing to clientSocket failed... {}", e),
        };
    }

    fn listen(&mut self) -> String
    {
        let mut input = String::new();
        match self.stream.read_to_string(&mut input)
        {
            Ok(size) => println!("read in {} bytes", size),
            Err(e) => panic!("writing to clientSocket failed... {}", e),
        };
        input
    }

}
