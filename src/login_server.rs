use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
};

use crate::{account::Account, network_message::NetworkMessage};

pub fn listen(addr: [u8; 4], port: u16) -> std::io::Result<()> {
    let addrs = [SocketAddr::from((addr, port))];

    let listener = TcpListener::bind(&addrs[..])?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_data(stream?)?;
    }

    Ok(())
}

fn handle_data(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buffer = [0; 256];
    let nread = stream.read(&mut buffer).unwrap();
    let buffer = &buffer[..nread];

    let mut msg = NetworkMessage::new(buffer.to_vec());
    msg.skip_bytes(2);
    let prot_id = msg.get_u16();
    if prot_id == 0x0201 {
        msg = get_authenticate_response(msg);
    } else if prot_id == 0x020A {
        msg.reset();
        msg.add_byte(0xff);
        stream.write_all(&msg.buffer)?;

        msg.reset();
        msg.add_byte(0xb4);
        msg.add_byte(0x12);
        msg.add_string("Hello world");
    } else {
        println!("Unknown package {:04x}", prot_id);
    }

    stream.write_all(&msg.buffer)?;

    Ok(())
}

fn get_authenticate_response(mut msg: NetworkMessage) -> NetworkMessage {
    msg.reset();
    msg.skip_bytes(17);
    let account_number = msg.get_u32();
    let account_passwd = msg.get_string();

    msg.reset();
    let account = Account::get(account_number, account_passwd);
    match account {
        Some(acc) => {
            msg.add_byte(0x64);

            msg.add_byte(acc.number_of_characters());

            for character in acc.iter() {
                msg.add_string(&character.nickname);
                msg.add_string(&character.server.name);
                msg.add_u32(u32::to_be(character.server.ip.into()));
                msg.add_u16(character.server.port);
            }

            msg.add_u16(acc.premium_days);
        }
        None => {
            msg.add_byte(0xa);

            msg.add_string("Invalid credentials");
        }
    }

    return msg;
}
