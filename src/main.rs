use capnp::{
    message::{Builder, ReaderOptions},
    serialize,
};
use serde::{Deserialize, Serialize};
use std::time::Instant;
use std::{env, fs};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

mod cats_capnp;

#[derive(Serialize, Deserialize, Debug)]
struct Address {
    street: String,
    number: u16,
    postalcode: u16,
}

#[derive(Serialize, Deserialize, Debug)]
struct Cat {
    name: String,
    age: u8,
    color: String,
    cuteness: f32,
    addresses: Vec<Address>,
    image: Option<Vec<u8>>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let img = read_img();
    let data = build_msg(&img);
    deserialize(&data);

    let json_data = build_msg_json(&img);
    deserialize_json(&json_data);

    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        match &args[1][..] {
            "c" => return client(&data, &json_data).await,
            "s" => return server().await,
            _ => (),
        }
    }

    Ok(())
}

async fn client(data: &[u8], json_data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    println!("started in CLIENT mode");

    let mut stream = TcpStream::connect("127.0.0.1:3000").await?;
    stream.write_all(data).await?;

    let mut stream = TcpStream::connect("127.0.0.1:3000").await?;
    stream.write_all(json_data).await?;

    Ok(())
}

async fn server() -> Result<(), Box<dyn std::error::Error>> {
    println!("started in SERVER mode");
    let listener = TcpListener::bind("127.0.0.1:3000").await?;

    println!("server running at 127.0.0.1:3000");

    loop {
        match listener.accept().await {
            Ok((mut socket, addr)) => {
                tokio::spawn(async move {
                    println!("accepted connection from: {}", addr);
                    let mut buf = [0; 1024];
                    let mut sum = 0;

                    loop {
                        let n = match socket.read(&mut buf).await {
                            Ok(n) => {
                                if n == 0 {
                                    println!("read {:?} bytes", sum);
                                    return;
                                } else {
                                    n
                                }
                            }
                            Err(e) => {
                                println!("error reading from socket: {}", e);
                                return;
                            }
                        };
                        sum += n;
                    }
                });
            }
            Err(e) => println!("error on client connection: {}", e),
        }
    }
}

fn read_img() -> Vec<u8> {
    fs::read("./minka.jpg").expect("can load image")
}

// JSON
fn build_msg_json(img: &[u8]) -> Vec<u8> {
    let mut addresses = vec![];
    for i in 0..10 {
        addresses.push(Address {
            street: String::from("some street"),
            number: i,
            postalcode: 1234,
        });
    }
    let cat = Cat {
        name: String::from("Minka"),
        age: 8,
        color: String::from("lucky"),
        cuteness: 100.0,
        addresses,
        image: Some(img.to_owned()),
        // image: None,
    };

    let start = Instant::now();
    let data = serde_json::to_vec(&cat).expect("can json serialize cat");
    let duration = start.elapsed();
    println!("JSON SE: duration: {:?}, len: {}", duration, data.len());
    data
}

fn deserialize_json(data: &[u8]) {
    let start = Instant::now();
    let cat: Cat = serde_json::from_slice(data).expect("can deserialize json");
    let duration = start.elapsed();
    println!("JSON DE: {}, duration: {:?}", cat.name, duration);
}

// CAPNPROTO

fn build_msg(img: &[u8]) -> Vec<u8> {
    let mut msg = Builder::new_default();

    let mut cat = msg.init_root::<cats_capnp::cat::Builder>();
    cat.set_name("Minka");
    cat.set_age(8);
    cat.set_color("lucky");
    cat.set_cuteness(100.0);
    cat.set_image(img);
    let mut addresses = cat.init_addresses(10);
    {
        for i in 0..10 {
            let mut address = addresses.reborrow().get(i);
            address.set_street("some street");
            address.set_number(i as u8);
            address.set_postalcode(1234);
        }
    }

    let start = Instant::now();
    let data = serialize::write_message_to_words(&msg);
    let duration = start.elapsed();
    println!("CAPNP SE: duration: {:?}, len: {}", duration, data.len());
    data
}

fn deserialize(data: &[u8]) {
    let start = Instant::now();

    let reader = serialize::read_message(data, ReaderOptions::new()).expect("can create reader");
    let cat = reader
        .get_root::<cats_capnp::cat::Reader>()
        .expect("can deserialize cat");

    let duration = start.elapsed();
    println!(
        "CAPNP DE: {} duration: {:?}",
        cat.get_name().unwrap(),
        duration
    );
}
