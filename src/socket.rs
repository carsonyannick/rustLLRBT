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
    pub id: u32,
    pub data: [u8;30],
}

impl Default for result
{
    fn default() -> result
    {
        result {
            command: [0;7],
            // argument: [0;10],
            id: 0,
            data: [0;30],
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
        let mut res = result::default();

        // 1) read command
        match self.stream.read_exact(&mut res.command)
        {
            Ok(()) => 
            {
                res.command[6] = b'\0';
                #[cfg(feature="debug")]
                {
                    let out = ::std::str::from_utf8(&res.command).unwrap();
                    println!("after first match (command) inside listen() {}", out);
                }

                ()
            }
            Err(e) =>
            {
                panic!("error reading clientSocket (command) failed... {}", e);
            }
        };

        // 2) read id
        let mut buf = [0;7];
        match self.stream.read_exact(&mut buf)
        {
            Ok(()) => 
            {
                let out = ::std::str::from_utf8(&buf[0..6]).unwrap();

                res.id = out.parse::<u32>().unwrap();

                #[cfg(feature="debug")]
                {
                    println!("before conversion to int, id {:?}", buf);
                    println!("after second match (id) inside listen() {}", res.id);
                }

            }
            Err(e) =>
            {
                panic!("error reading clientSocket (id) failed... {}", e);
            }
        };

        // 3) length of data
        let mut buf = [0;2];
        let dataLength: usize;
        match self.stream.read_exact(&mut buf)
        {
            Ok(()) => 
            {
                let mut out = ::std::str::from_utf8(&buf).unwrap();

                dataLength = out.parse::<usize>().unwrap();

                #[cfg(feature="debug")]
                {
                    println!("before conversion to int, dataLength {:?}", buf);
                    println!("after (dataLength) inside listen() {}", dataLength);
                }

            }
            Err(e) =>
            {
                panic!("error reading clientSocket (dataLength) failed... {}", e);
            }
        };

        // 4) read data
        // match self.stream.read_exact(&mut res.data)
        match self.stream.read_exact(&mut res.data[0..(dataLength)])
        {
            Ok(()) => 
            {
                // res.data[29] = 0;
                // res.data[dataLength] = b'\0';

                #[cfg(feature="debug")]
                {
                    let out = ::std::str::from_utf8(&res.data).unwrap();
                    println!("after third match (data) inside listen() {}", out);
                }

                return res
            }
            Err(e) =>
            {
                panic!("error reading clientSocket (data) failed... {}", e);
            }
        };
    }
}
