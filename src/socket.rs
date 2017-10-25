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
            Ok(sock) => return server{socket:sock},
            Err(e) => 
            {
                panic!("Couldn't connect: {}", e);
            }
        };
    }

    // pub fn accept(&self) -> clientSocket
    // {
    //     println!("Inside accept()");
    //     match self.socket.accept() 
    //     {
    //         Ok((clientStream, clientAddr)) => return clientSocket{stream: clientStream},
    //         Err(e) => panic!("accept function failed.. {}", e),
    //     };
    // }

    pub fn accept(&self) -> clientSocket
    {
        // println!("Inside accept()");
        match self.socket.accept() 
        {
            Ok((clientStream, clientAddr)) => 
            {
                // clientStream.set_read_timeout(Some(::std::time::Duration::new(0,200)));
                return clientSocket{stream: clientStream}
            }
            Err(e) => panic!("accept function failed.. {}", e),
        };
    }

}

pub struct result
{
    pub command:  [u8;7],
    // pub argument: [u8;10],
    pub argument: u32,
}

impl Default for result
{
    fn default() -> result
    {
        result {
            command: [0;7],
            // argument: [0;10],
            argument: 0,
        }
    }
}

impl result
{
    pub fn command_is(&self, other: &[u8]) -> bool
    {
        // other.iter().zip(self.command.iter()).all(|(a,b)| a == b)
        self.command.iter().zip(other.iter()).all(|(a,b)| a == b)
    }
}
    

pub struct clientSocket
{
    stream: UnixStream,
}

impl clientSocket
{

    pub fn send(&mut self, output: String)
    {
        match self.stream.write_all(output.as_bytes())
        {
            Ok(nothing) => {},
            Err(e) => panic!("writing to clientSocket failed... {}", e),
        };
    }

    pub fn listen(&mut self) -> result
    {
        let mut buf = [0;10];
        let mut res = result::default();
        // res.command = [0;6];

        match self.stream.read_exact(&mut res.command)
        {
            Ok(()) => 
            {
                ()
            }
            Err(e) =>
            {
                // panic!("error reading clientSocket (command) failed... {}", e);
            }
        };
        // println!("after first match inside listen()");

        // match self.stream.read_exact(&mut res.argument)
        match self.stream.read_exact(&mut buf)
        {
            Ok(()) => 
            {
                // println!("buf {:?}", buf);
                let out_ = ::std::str::from_utf8(&buf[0..8]).unwrap();
                println!("buf str {:?}", out_);
                res.argument = out_.parse::<u32>().unwrap();
                // let mut out = &buf[..];
                // let out_ = out.read_u32::<LittleEndian>().unwrap();
                println!("after second match inside listen() {}", out_);
                return res
            }
            Err(e) =>
            {
                panic!("error reading clientSocket (argument) failed... {}", e);
            }
        };
    }
}
