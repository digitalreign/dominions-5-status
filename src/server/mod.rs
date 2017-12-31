use byteorder::{LittleEndian, WriteBytesExt, ReadBytesExt};
use hex_slice::AsHex;
use flate2::read::ZlibDecoder;
use std::io::{Cursor, Read, Write};
use std::io;
use std::net;

#[repr(C)]
#[derive(Debug)]
pub struct RawGameData {
    pub a: [u8; 6], // 6
    pub game_name: String,
    pub c: [u8; 6], // 6
    pub d: u32, // 4
    pub e: u8,  // 1
    pub f: Vec<u8>, // ; 750],
    pub g: u8,  // 1
    pub h: u32, // 4
    pub i: u32, // 4
    pub j: u8,  // 1
}

pub fn get_game_data(server_address: &String) -> io::Result<RawGameData> {
    let buffer = call_server_for_info(server_address)?;
    let decompressed = decompress_server_info(&buffer)?;
    let game_data = parse_data(&decompressed)?;
    println!("data: {:?}", game_data);
    Ok(game_data)
}

fn call_server_for_info(server_address: &String) -> io::Result<Vec<u8>> {
    println!("starting");
    let mut stream = net::TcpStream::connect(server_address)?;
    // let mut stream = net::TcpStream::connect("dom5.snek.earth:30028")?;
    println!("connected");
    let mut wtr = vec![];
    wtr.write_u8(b'f')?;
    wtr.write_u8(b'H')?;
    wtr.write_u32::<LittleEndian>(1)?;
    wtr.write_u8(3)?;

    println!("Sending {:x}", wtr.as_slice().as_hex());
    let _ = stream.write(&mut wtr)?;
    println!("sent");
    let mut buffer = [0; 512];
    println!("trying to receive");
    let _ = stream.read(&mut buffer)?;
    println!("Received {:x}", buffer.as_hex());

    let mut wtr2 = vec![];
    wtr2.write_u8(b'f')?;
    wtr2.write_u8(b'H')?;
    wtr2.write_u32::<LittleEndian>(1)?;
    wtr2.write_u8(11)?;
    println!("Sending {:x}", wtr2.as_slice().as_hex());
    let _ = stream.write(&mut wtr2)?;
    println!("sent");

    Ok(buffer.to_vec())
}

fn decompress_server_info(raw: &[u8]) -> io::Result<Vec<u8>> {
    assert!(raw[1] == b'J');
    let mut decoder = ZlibDecoder::new(&raw[10..]);
    let mut decompressed = vec![];
    let _ = decoder.read_to_end(&mut decompressed)?;
    println!("{:x}", decompressed.as_slice().as_hex());
    Ok(decompressed)
}

fn parse_data(data: &[u8]) -> io::Result<RawGameData> {
    let game_name_len = data.len() - 27 - 750; // Possibly null terminated?
    let mut cursor = Cursor::new(data);
    let mut a = [0u8; 6]; 
    cursor.read(&mut a)?;
    
    let mut game_name_buff = vec![0u8; game_name_len];
    cursor.read_exact(&mut game_name_buff)?;
    let game_name = String::from_utf8_lossy(&game_name_buff).to_string();

    let mut c = [0u8; 6];
    cursor.read(&mut c)?;
    
    let d = cursor.read_u32::<LittleEndian>()?;

    let e = cursor.read_u8()?;

    let mut f = vec![0u8; 750];
    cursor.read_exact(&mut f)?;

    let g = cursor.read_u8()?;

    let h = cursor.read_u32::<LittleEndian>()?;

    let i = cursor.read_u32::<LittleEndian>()?;

    let j = cursor.read_u8()?;
    assert!(cursor.position() as usize == cursor.get_ref().len());

    Ok(RawGameData {
        a: a,
        game_name: game_name,
        c: c,
        d: d,
        e: e,
        f: f,
        g: g,
        h: h,
        i: i,
        j: j,
    })
}