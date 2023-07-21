use capnp::{
    message::{Builder, ReaderOptions},
    serialize,
};
use std::fs;

mod cats_capnp;

fn main() {
    // TODO: build server that broadcasts events, build client
    println!("Hello, world!");
    let img = read_img();
    let data = build_msg(&img);
    deserialize(&data);
}

fn read_img() -> Vec<u8> {
    fs::read("./minka.jpg").expect("can load image")
}

fn build_msg(img: &[u8]) -> Vec<u8> {
    let mut msg = Builder::new_default();

    let mut cat = msg.init_root::<cats_capnp::cat::Builder>();
    cat.set_name("Minka");
    cat.set_age(8);
    cat.set_color("lucky");
    cat.set_cuteness(100.0);
    // cat.set_image(img);

    let data = serialize::write_message_to_words(&msg);
    println!("data: {:?}", data);
    data
}

fn deserialize(data: &[u8]) {
    let reader = serialize::read_message(data, ReaderOptions::new()).expect("can create reader");
    let cat = reader
        .get_root::<cats_capnp::cat::Reader>()
        .expect("can deserialize cat");
    println!("cat: {:?}", cat);
}
