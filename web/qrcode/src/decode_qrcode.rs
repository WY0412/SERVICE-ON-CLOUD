use crate::encode_qrcode;
mod find_qrcode;
use crate::utils;

use serde::{Serialize, Deserialize};
const SUCCESS_RESPONSE: &str = "CCprojectFor3,851725245278\n";

#[derive(Serialize, Deserialize, Debug)]
struct QrcodeVerifyResponse {
    message: String,
    token: String,
}

// Define a struct that represents your data structure
#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    team_name: String,
    timestamp: u64, // Assuming timestamp is a string; change type as needed
    decoded_qrcode: String,
}


pub async fn get_verify_response(client: &reqwest::Client, hex_message: &str, timestamp: &str) ->  Result<String, reqwest::Error> {
    let start = std::time::Instant::now();
    let decoded_data = decode(hex_message, timestamp);
    println!("Time used to decode(hex_message, timestamp): {:?}", start.elapsed());
    let decoded_json = serde_json::to_string(&decoded_data).unwrap();
    let start = std::time::Instant::now();
    let res = client.post("http://127.0.0.1:9000/rest_auth")
        .header("Content-Type", "application/json")
        .body(decoded_json)
        .send()
        .await?;

    print!("Time used to send request: {:?}", start.elapsed());
    let start = std::time::Instant::now();
    let response_data: QrcodeVerifyResponse = res.json().await?;
    println!("Time used to get QrcodeVerifyResponse: {:?}", start.elapsed());
    if response_data.message != "Success" {
        print!("Error: {}", response_data.message);
    }
    return Ok(SUCCESS_RESPONSE.to_string() + &response_data.token);
}

pub fn decode(hex_message: &str, timestamp: &str) -> Data{
    let mut decoded_matrix = str_to_32_by_32_matrix(hex_message);
    encode_qrcode::xor_with_logistic_map(&mut decoded_matrix);
    let qr_code = find_qrcode::find_qr_code(&decoded_matrix);
    let matrix_dim = qr_code.len();
    let str_length: usize;
    let order_arr: &[&[usize]];
    if matrix_dim == 21 {
        str_length = 224;
        order_arr = &utils::ORDER_21;
    } else {
        str_length = 370;
        order_arr = &utils::ORDER_25;
    }
    let mut char_string = String::new(); // This will store the final char string
    let mut bit_buffer = 0u32; // Buffer for accumulating bits
    let mut bits_collected = 0; // Number of bits collected in the buffer
    // first byte is the length of the string
    let mut is_first = true;
    let mut is_correcting_byte = false;
    let mut true_length: usize = 0;

    for ind in 0..str_length {
        if bits_collected == 8 {
            if is_first {
                true_length = bit_buffer as usize;
                is_first = false;
            } else {
                if is_correcting_byte {
                    is_correcting_byte = false;
                } else {
                    char_string.push_str(&(bit_buffer as u8 as char).to_string());
                    is_correcting_byte = true;
                    if char_string.len() == true_length {
                        break;
                    }
                }
            }
            bits_collected = 0;
            bit_buffer = 0;
        }
        let (y, x) = (order_arr[ind][0], order_arr[ind][1]);
        bit_buffer = (bit_buffer << 1) | (qr_code[y][x] as u32);
        bits_collected += 1;
    }

    let data = Data {
        team_name: "CCprojectFor3".to_string(),
        timestamp: timestamp.parse().unwrap(),
        decoded_qrcode: char_string,
    };
    data

}

fn str_to_32_by_32_matrix(message: &str) -> Vec<Vec<u8>> {
    // Find indices of all "0x"

    // Create a single string from all hex strings
    let start = std::time::Instant::now();
    let mut binary_string = String::new();
    let hex_strings = message.split("0x").skip(1);
    let mut qr_code: Vec<Vec<u8>> = vec![vec![0;32];32];

    for hex_str in hex_strings {
        // Pad the hex string to ensure it is 8 characters long
        let padded_hex = format!("{:0>8}", hex_str); // Pad with zeros on the left
        let bin = format!("{:032b}", u32::from_str_radix(&padded_hex, 16).unwrap()); // Convert to binary
        binary_string.push_str(&bin);
    }

    assert!(binary_string.len() == 1024, "Binary string length is not 1024: {}", binary_string.len());
    let bits: Vec<u8> = binary_string.chars().map(|c| c as u8 - b'0').collect();

    for i in 0..32 {
        for j in 0..32 {
            let ind = i * 32 + j;
            // Convert '0' or '1' to 0 or 1
            qr_code[i][j] = bits[ind];
        }
    }
    println!("Time used to str_to_32_by_32_matrix: {:?}", start.elapsed());

    return qr_code;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_decode(){
        let decode_data = "0x663cbc630x98f78b370xbef2db180xddd114aa0x54a0e1d70xe9a244870x1659c7170x89506acf0x4dc10b180xefdf26e0xe8e005e10x86a4fda90x15ee1b320xa0ee170e0xe461c3740x928e7f340x98698d070x239af4b60x627807440xfd89c1300x422764790xcd1ac7250xf4364a650x48eeb1570xa7946ab10x665176370x81a511290xceaf2a070xd3fdb0060xebf8889f0x9f7c49160x69e8175b";
        let timestamp = "1706567290739";
        let json_data: Data = decode(decode_data, timestamp);
        print!("{:?}", json_data);
    }

    #[ntex::test]
    async fn test_get_verify_response(){
        let decode_data = "0x663cbc630x98f78b370xbef2db180xddd114aa0x54a0e1d70xe9a244870x1659c7170x89506acf0x4dc10b180xefdf26e0xe8e005e10x86a4fda90x15ee1b320xa0ee170e0xe461c3740x928e7f340x98698d070x239af4b60x627807440xfd89c1300x422764790xcd1ac7250xf4364a650x48eeb1570xa7946ab10x665176370x81a511290xceaf2a070xd3fdb0060xebf8889f0x9f7c49160x69e8175b";
        let timestamp = "1706567290739";
        let client = reqwest::Client::new();

        let response = get_verify_response(&client, decode_data, timestamp);
        print!("\ntest ok, {:?}", response.await.unwrap());
    }
}