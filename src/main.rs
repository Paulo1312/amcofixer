use std::{io, string::FromUtf8Error};
use std::fs::File;
use std::env;
use std::io::prelude::*;

use serde_json::Value;

pub fn decode_url_base64_str(url_string: String) -> Result<String, FromUtf8Error>{
    use base64::{Engine as _, alphabet, engine::{self, general_purpose}};
    let bytes_url = engine::GeneralPurpose::new(
                 &alphabet::URL_SAFE,
                 general_purpose::NO_PAD)
        .decode(url_string).unwrap();
    String::from_utf8(bytes_url)
}

pub fn decode_url_base64_byte(url_string: String) -> Result<Vec<u8>, base64::DecodeError> {
    use base64::{Engine as _, alphabet, engine::{self, general_purpose}};
    engine::GeneralPurpose::new(
                 &alphabet::URL_SAFE,
                 general_purpose::NO_PAD).decode(url_string)
}

pub fn encode_url_base64_byte(data: Vec<u8>) -> String{ 
    use base64::{Engine as _, alphabet, engine::{self, general_purpose}};

    engine::GeneralPurpose::new(
        &alphabet::URL_SAFE,
        general_purpose::NO_PAD).encode(data)
}
pub fn decode_zlib(data_to_decode: &[u8]) -> io::Result<String>{
    use flate2::read::ZlibDecoder;
    use std::io::prelude::*;

    let mut z = ZlibDecoder::new(&data_to_decode[..]);
    let mut s = String::new();
    z.read_to_string(&mut s)?;
    Ok(s)
}

pub fn encode_zlib(data: String) -> Result<Vec<u8>, io::Error> {
    use std::io::prelude::*;
    use flate2::Compression;
    use flate2::write::ZlibEncoder;

    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    e.write_all(data.as_bytes())?;
    e.finish()
}

pub fn cut_config(data1: String, cut_number: usize) -> String{
    data1[cut_number..data1.len()].to_string()
}

pub fn cut_config_byte(data1: &[u8], cut_number: usize) -> &[u8]{
  &data1[cut_number..data1.len()]
}

fn json_fix(data: String) -> String{
    let a: Value = serde_json::from_str(&data).unwrap();
    let last_conf = a["containers"][0]["cloak"]["last_config"].to_string().replace("\\\"", "\"");
    let last_conf1 = &last_conf[1..last_conf.len() -1];
    //let last_config: Value = serde_json::from_str(&last_conf.replace("\\\"", "\"")).unwrap();
    let last_config: Value = serde_json::from_str(&last_conf1).unwrap();
    let openvpn_prep = &a["containers"][0]["openvpn"]["last_config"].to_string().replace("\\\\n", "\n").replace("\\\"", "\"");

    let openvpn = openvpn_prep;
    //let openvpn: Value = serde_json::from_str(&openvpn_prep[1..openvpn_prep.len()-1]).unwrap();
    //println!("{}", a["containers"][0]["openvpn"]);
   // println!("RemoteHost: {}", last_config["RemoteHost"]);
    let route = openvpn.split("route ").last().unwrap().split(" 255.255.255.255").nth(0).unwrap();
    let remote_host = last_config["RemoteHost"].to_string().replace("\"", "");
    //println!("\n========================\nRoute: {}\n========================", route);
    //println!("RemoteHost: {}\n========================", remote_host);
    let new_data = data.replace(&remote_host,  route);
    //println!("\n\n\n{}",  new_data);
    new_data
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1{
        let file_name = args[1].clone();
        match File::open(&file_name){
            Ok(mut file1) => {
                let mut data1 = String::new();
                match file1.read_to_string(&mut data1) {
                    Ok(_) => {
                        if data1.contains("vpn://"){
                            if data1.find("vpn://").unwrap() == 0_usize {
                                data1 = data1.replace("vpn://", "")
                            }
                        }
                        if data1.ends_with("\n") {
                            data1 = data1.replace("\n","");
                        }
                        //println!("{:?}",data1);
                        let data2 = decode_url_base64_byte(data1.to_string()).unwrap();
                        let debase64 = cut_config_byte(&data2, 4);
                        let decode64 = decode_zlib(debase64).unwrap();
                  //      println!("========================================================================================\n\nData: \n{:?}\n\n========================================================================================\n\nDecode Data: \n\n{}", debase64, decode64);
                        let fixed_data = json_fix(decode64);
                        let mut encoded_data = encode_zlib(fixed_data).unwrap();
                        encoded_data.reverse();
                        encoded_data.push(255);
                        encoded_data.push(0);
                        encoded_data.push(0);
                        encoded_data.push(0);
                        encoded_data.reverse();
                    //    print!("{:?}", encoded_data);
                        let encoded_data1 = encode_url_base64_byte(encoded_data);
                        // println!("\n\n========================================================================================\n{}", encoded_data1);
                        match File::create(format!("new_{}", file_name)) {
                            Ok(mut file2) => {
                                match file2.write_all(format!("vpn://{}\n", &encoded_data1).as_bytes()) {
                                    Ok(_) => (),
                                    Err(err) => print!("error: {}", err)
                                };
                            }
                            Err(err) => println!("Err: {}", err)
                        }
                    }
                    Err(err) => print!("{}", err)
                }
            }
            Err(err) => println!("{}", err)
        }
    }
    else{
        println!("Error(")
    }
    // Decode
    

}
