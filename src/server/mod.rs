// A tiny async echo server with tokio-core
extern crate futures;
extern crate tokio_core;
extern crate tokio_io;

use self::futures::{Future, Stream};
use self::tokio_io::{io, AsyncRead};
use self::tokio_core::net::TcpListener;
use self::tokio_core::reactor::Core;


use std::{thread,time};



static SERVER_ADDRESS: &'static str = "127.0.0.1:12345";



pub fn run_server() {
    // Create the event loop that will drive this server
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    // Bind the server's socket
    let addr = SERVER_ADDRESS.parse().unwrap();
    let tcp = TcpListener::bind(&addr, &handle).unwrap();

    // Iterate incoming connections
    let server = tcp.incoming().for_each(|(tcp, _)| {
        // Split up the read and write halves
        let (reader, writer) = tcp.split();

        // Future of the copy
        let bytes_copied = io::copy(reader, writer);

        // ... after which we'll print what happened
        let handle_conn = bytes_copied.map(|(n, _, _)| {
            println!("wrote {} bytes", n)
        }).map_err(|err| {
            println!("IO error {:?}", err)
        });

        // Spawn the future as a concurrent task
        handle.spawn(handle_conn);

        Ok(())
    });

    // Spin up the server on the event loop
    core.run(server).unwrap();
}



#[test]
#[ignore]
fn test_run_server() {
    thread::spawn(|| { run_server(); });
    thread::sleep(time::Duration::from_millis(3000));
}
