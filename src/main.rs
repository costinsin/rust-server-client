use clap::{App, Arg};
use std::{
    io::{stdin, Error, ErrorKind, Read, Result, Write},
    net::{TcpListener, TcpStream},
    str::from_utf8,
    thread,
};

fn main() -> Result<()> {
    let matches = App::new("TCP Server-Client")
        .arg(
            Arg::with_name("server")
                .long("server")
                .help("Use this option to run the app as a server")
                .takes_value(false)
                .conflicts_with("client"),
        )
        .arg(
            Arg::with_name("client")
                .long("client")
                .help("Use this option to run the app as a client")
                .takes_value(false)
                .conflicts_with("server"),
        )
        .get_matches();

    if matches.is_present("client") {
        // Handle client side connection
        let mut stream = TcpStream::connect("127.0.0.1:5000")?;

        loop {
            let mut buf = [0; 1024];

            stdin().read(&mut buf)?;
            stream.write(&buf)?;
        }
    } else if matches.is_present("server") {
        // Handle server side connection
        let listener = TcpListener::bind("127.0.0.1:5000")?;

        loop {
            let (mut stream, sock) = listener.accept()?;
            println!(
                "New connection accepted from {}:{}!",
                sock.ip(),
                sock.port()
            );

            thread::spawn(move || -> Result<()> {
                let mut buf = [0; 1024];

                loop {
                    let read_bytes = stream.read(&mut buf)?;

                    if read_bytes == 0 {
                        break;
                    }

                    print!(
                        "{}",
                        from_utf8(&buf).map_err(|_| Error::new(
                            ErrorKind::InvalidData,
                            "UTF-8 convertion failed!"
                        ))?
                    );
                }

                println!("Connection {}:{} dropped!", sock.ip(), sock.port());

                Ok(())
            });
        }
    }

    Ok(())
}
