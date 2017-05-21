



use std::{thread,time};
use server;
use client;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_client_server() {
        println!("testing client with server");


        thread::spawn(|| { server::run_server(); });


        thread::sleep(time::Duration::from_millis(3000));

        client::ping();

        thread::sleep(time::Duration::from_millis(1000));

    }
}
