// use crate::utils; // Import the utils module
// mod utils;

use crate::utils;
use crate::matrix;

const FILLER: u16 = 0b1110110000010001;
const SUCCESS_RESPONSE: &str = "CCprojectFor3,851725245278\n";

fn get_matrix_dim(message: &str) -> usize {
    if message.len() <= 13 {
        return 21;
    } else if message.len() <= 22 {
        return 25;
    } else {
        panic!("Message is too long to encode in a QR code");
    }
}
fn create_bytes_from_raw_string(message: &str) -> Vec<u8> {
    // Filter the message to keep only ASCII alphanumeric characters
    // Initialize the payload with the length of the filtered message
    let mut payload = Vec::new();
    let message_length = message.len() as u8;
    payload.push(message_length);

    // Append the message in ASCII format and its error correction byte
    for char in message.bytes() {
        payload.push(char); // Add the ASCII character

        // Calculate and append the error correction byte
        let error_code = (char.count_ones() % 2) as u8; // Calculate parity bit
        payload.push(error_code); // In our simplified version, this is a full byte
    }
    return payload;
}

fn create_qr_matrix_from_bytes(payload: &[u8], dim: usize) -> Vec<Vec<u8>> {
    // Create a QR code matrix with the payload
    let mut qr_code: Vec<Vec<u8>> = vec![vec![0; dim]; dim]; // 21x21 QR code
    let mut filler_index = 0;
    // Add the payload to the QR code matrix
    let order_arr: &[&[usize]];
    let qr_template: &[&[usize]];
    let length_to_fill;
    let matrix_posiiton_fn: fn(u64) -> matrix::Position;
    if dim == 21 {
        order_arr = &utils::ORDER_21;
        qr_template = &utils::QR_TEMPLATE_21;
        length_to_fill = 224;
        matrix_posiiton_fn = matrix::get_matrix_position_version1;
    } else {
        order_arr = &utils::ORDER_25;
        qr_template = &utils::QR_TEMPLATE_25;
        length_to_fill = 370;
        matrix_posiiton_fn = matrix::get_matrix_position_version2;
    }
    for ind in 0..length_to_fill {
        let char_ind = ind / 8;
        let bit_ind = ind % 8;
        let (y, x) = (order_arr[ind][0], order_arr[ind][1]);
        let pos = matrix_posiiton_fn(ind as u64);
        let (x2, y2) = (pos.col as usize, pos.row as usize);
        if (y != y2) || (x != x2) {
            panic!("Error in position pattern");
        }
        if char_ind >= payload.len() {
            qr_code[y][x] = ((FILLER >> (15 - filler_index)) & 1) as u8;
            filler_index = (filler_index + 1) % 16;
        } else {
            qr_code[y][x] = (payload[char_ind] >> (7 - bit_ind)) & 1;
        }
    }
    // Add the QR code template to the QR code matrix
    for i in 0..qr_template.len() {
        for j in 0..qr_template[i].len() {
            if qr_template[i][j] == 1 {
                qr_code[i][j] = 1;
            }
        }
    }
    return qr_code;
}

pub fn xor_with_logistic_map(qr_code: &mut Vec<Vec<u8>>) {
    let mut x: f64 = 0.1; // Initial value for logistic map
    const R: f64 = 4.0; // Logistic map parameter
    let mut logistic_value: u8 = (x * 255.0).floor() as u8; // Convert to byte
    let mut bit_counter = 0; // Counts up to 8
    let dim = qr_code.len();
    let mut is_first = true;

    for y_index in 0..dim {
        for x_index in 0..dim { // Loop through each bit in the QR code matrix
            // Generate new logistic_value every 8 bits
            if bit_counter % 8 == 0 && !is_first {
                x = R * x * (1.0 - x); // Update logistic map value
                bit_counter = 0; // Reset bit counter
                logistic_value = (x * 255.0).floor() as u8; // Convert to byte
            }
            is_first = false;

            // Extract the current bit from logistic_value
            let logistic_bit = (logistic_value >> bit_counter) & 1;

            // XOR the QR code bit with the current logistic map bit
            let xor_bit = qr_code[y_index][x_index] ^ logistic_bit;

            // Set the new XORed bit back into the QR code
            qr_code[y_index][x_index] = xor_bit;

            bit_counter += 1;

        }
    }
}


fn qr_code_matrix_to_hex_string(qr_code: &Vec<Vec<u8>>) -> String {
    let mut hex_string = String::new(); // This will store the final hex string
    let mut bit_buffer = 0u32; // Buffer for accumulating bits
    let mut bits_collected = 0; // Number of bits collected in the buffer

    // Iterate through each row and then each column in the QR code matrix
    for row in qr_code.iter() {
        for &bit in row.iter() {
            // Shift the buffer left by one and add the current bit
            bit_buffer = (bit_buffer << 1) | (bit as u32);
            bits_collected += 1;

            // Check if we've filled up 32 bits
            if bits_collected == 32 {
                hex_string += format_bit_buffer(bit_buffer).as_str(); // Add the hexadecimal string
                // hex_string += &format!("{:08X}", bit_buffer);
                bit_buffer = 0; // Reset the buffer
                bits_collected = 0; // Reset the bit counter
            }
        }
    }

    // Handle any remaining bits by shifting them to match the MSB representation
    if bits_collected > 0 {
        // Shift the remaining bits to the left to fill up the 32 bits
        // Append the remaining bits to the hex string
        hex_string += format_bit_buffer(bit_buffer).as_str();
    }

    return hex_string
}

fn format_bit_buffer(bit_buffer: u32) -> String {
    // Convert the 32-bit buffer to a hexadecimal string, each start with 0x prefix
    let bit_buffer_hex = format!("{:08X}", bit_buffer).to_lowercase();
    // Remove leading zeros from the hexadecimal string
    let bit_buffer_without_leading_zeros = bit_buffer_hex.trim_start_matches('0');
    let mut hex_string = String::new();
    hex_string += "0x"; // Add the 0x prefix
    if bit_buffer_without_leading_zeros.is_empty() {
        hex_string += "0"; // Add a single zero if the buffer is zero
    } else {
        hex_string += bit_buffer_without_leading_zeros; // Add the hexadecimal string
    }
    return hex_string;
}

pub fn encode(message: &str) -> String{
    let payload = create_bytes_from_raw_string(&message);
    let dim = get_matrix_dim(&message);
    let mut qr_code = create_qr_matrix_from_bytes(&payload, dim);
    xor_with_logistic_map(&mut qr_code);
    let hex_string = qr_code_matrix_to_hex_string(&qr_code);
    return SUCCESS_RESPONSE.to_string() + &hex_string;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_encode_25(){
        let message = "CC Team is awesome!";
        let hex_string = encode(message);
        assert!(hex_string == "0x66ede8530xb3b981a10xed18e4040xa4a0026c0xd039db570x21976f0d0xed168440xfdce22bf0xd67e47ec0x2171a0600x2a1a95010x875f3f480x78347f130x886ccc430xc90f439a0x331f54900x7bbcbf030x20d731250xc555223e0x15858");
        print!("{:?}", hex_string);
    }

    #[test]
    fn test_encode_21(){
        let message = "CC Team";
        let hex_string = encode(message);
        assert!(hex_string == "0x66d92b800x5bc76d830x121a7fa60x51c111870x3a5f3ca30x8be36a130xedb223a0xfc8e98780x33bf50de0x2e8709700x545a2d0f0xecef7ae0x461175cd0xff132a");
        print!("{:?}", hex_string);
    }
}