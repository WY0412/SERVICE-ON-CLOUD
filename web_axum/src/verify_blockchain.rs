use flate2::write::ZlibDecoder;
use std::io;
use std::io::prelude::*;
use std::string::ToString;
use flate2::{write::ZlibEncoder, Compression};

extern crate base64;
extern crate num;

use base64::{alphabet, engine, Engine as _};
use base64::engine::general_purpose::URL_SAFE;
use serde::{Deserialize, Serialize};
use num::{BigInt, Num, ToPrimitive};
use sha256::digest;
use substring::Substring;

// json parsing structures
#[derive(Serialize, Deserialize)]
#[derive(Clone)]
struct Transaction {
    #[serde(skip_serializing_if = "Option::is_none")]
    sig: Option<u64>,
    recv: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    fee: Option<u64>,
    amt: u64,
    time: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    send: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hash: Option<String>
}

#[derive(Serialize, Deserialize)]
struct Block {
    all_tx: Vec<Transaction>,
    pow: String,
    id: u64,
    hash: String,
    target: String
}

#[derive(Serialize, Deserialize)]
struct Request {
    chain: Vec<Block>,
    new_target: String,
    new_tx: Vec<Transaction>
}

struct TxVerifyResult {
    time: String,
    transaction_hashes: String,
    validity: bool
}

const MALFORMED_RESPONSE: &str = "CCprojectFor3,851725245278\nINVALID";
const SUCCESS_RESPONSE: &str = "CCprojectFor3,851725245278\n";
const NON_APPLICABLE: &str = "N\\A";
const AMT: u64 = 500000000;
const TEN_MINUTES: u64 = 600000000000;
const N: u64 = 1561906343821;
const E: u64 = 1097844002039;
const D: u64 = 343710770439;

// find type helper
// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }

// time comparison
fn is_less_than(a: &String, b: &String) -> bool {
    return if a.len() != b.len() {
        a.len() < b.len()
    } else {
        a < b
    }
}

// add ten minutes to time
fn add_ten_minutes(a: &String) -> String {
    match a.parse::<BigInt>() {
        Ok(mut n)  => {
            n += TEN_MINUTES;
            return n.to_string();
        },
        Err(_) => println!("Error")
    }
    return NON_APPLICABLE.to_string()
}


pub fn validate_encoded_json(encoded_json: &String) -> String {
    let bytes_url_result = engine::GeneralPurpose::new(&alphabet::URL_SAFE, Default::default()).decode(encoded_json);
    let bytes_url;
    match bytes_url_result {
        Ok(bytes) => bytes_url = bytes,
        Err(_) => {
            return format!("{}|{}", MALFORMED_RESPONSE, "|0, URL decoding error");
        }
    }
    let json_str_result:Result<String, io::Error> = decode_reader(bytes_url);
    let json_str;
    match json_str_result {
        Ok(str) => json_str = str,
        Err(_) => {
            return format!("{}|{}", MALFORMED_RESPONSE, "|1, JSON decoding error");
        }
    }
    let response_str = validate_json(json_str);
    return response_str;
}

// validate all the json data and make modifications on it for further response
pub fn validate_json(json_str: String) ->  String {
    let request_result:Result<_, serde_json::Error>  = serde_json::from_str(&*json_str);
    let mut request;
    match request_result {
        Ok(req) => request = req,
        Err(_) => {
            return format!("{}|{}", MALFORMED_RESPONSE, "|2, invalid fields in JSON");
        }
    }
    // check the validity of previous blocks, and add new tx
    if !is_request_valid(& mut request) {
        return MALFORMED_RESPONSE.to_string();
    }
    let block_chain_str = serde_json::to_string(& request).unwrap();
    let compress_vec = compress_string(&block_chain_str).unwrap();
    let response_str = SUCCESS_RESPONSE.to_string() + &URL_SAFE.encode(compress_vec);
    return response_str;
}

// validate all blocks and add new blocks
fn is_request_valid(request: & mut Request) -> bool {
    let mut prev_time= "".to_string();

    if !is_chain_valid(&request.chain) {
        return false;
    }
    // add new tx with verifications
    let cur_block = &mut request.new_tx;
    let next_block_id = request.chain.len();
    let result: TxVerifyResult = validate_and_complete_all_new_tx(& mut prev_time, cur_block);
    let last_block_time = &request.chain[next_block_id-1].all_tx.last().unwrap().time;
    let reward_hash = add_reward_transaction(cur_block, &last_block_time, next_block_id as u64);

    let transaction_hashes = result.transaction_hashes + "|" + &reward_hash;

    if !result.validity {
        return false;
    }
    //find pow that's less than new target from 1 to inf
    let prev_block_hash = &request.chain[next_block_id-1].hash;
    let mut pow;
    let mut i = 0;
    let mut new_block_hash;
    loop {
        pow = i.to_string();
        new_block_hash = calculate_block_hash(next_block_id as u64, prev_block_hash, &transaction_hashes, &pow);
        if new_block_hash < request.new_target {
            break;
        }
        i += 1;
    }
    // append new block to chain
    let new_block = Block{all_tx: request.new_tx.clone(), pow: pow, id: next_block_id as u64, hash: new_block_hash, target: request.new_target.clone()};
    request.chain.push(new_block);

    // all the blocks pass verification with a new block added
    return true;
}

fn is_chain_valid(chain: & Vec<Block>) -> bool {
    let length = chain.len();
    let mut cur_block;
    let mut prev_block_hash= &"00000000".to_string();
    let mut prev_time= "".to_string();

    // iterate over each block except new tx
    for i in 0..length {
        cur_block = &chain[i];
        // validate all tx before new tx first
        let result = validate_all_tx(&prev_time, &cur_block.all_tx, i as u64);
        let validated_time  = result.time;
        let transaction_hashes = result.transaction_hashes;
        let is_all_tx_valid = result.validity;
        let is_block_id_valid: bool = cur_block.id == i as u64;

        if !is_all_tx_valid || !is_block_id_valid {
            return false;
        }

        // let hash_source = hash_source + &cur_block.id.to_string() + &bar + prev_block_hash + &bar + &transaction_hashes;
        let block_hash = calculate_block_hash(cur_block.id, prev_block_hash, &transaction_hashes, &cur_block.pow);
        // validate hash
        if !block_hash.eq(&cur_block.hash)|| !(block_hash < cur_block.target) {
            return false;
        }
        // update prev_time and prev_block_hash
        prev_time = validated_time;
        prev_block_hash = &cur_block.hash;
    }
    return true;
}

fn calculate_block_hash(block_id: u64, prev_block_hash: &String, transaction_hashes: &String, pow: &String) -> String {
    let sha256_source = format!("{}|{}{}", block_id, prev_block_hash, transaction_hashes);
    let sha256_hash = digest(sha256_source);
    let cc_hash_source = sha256_hash + pow;
    let new_sha256_hash = digest(cc_hash_source);
    let cc_hash = new_sha256_hash.substring(0,8).to_string();
    return cc_hash;
}

// validate all_tx in one single block, including new_tx
// return: time: String, transaction_hashes: String, validity: bool
fn validate_all_tx(time: &String, tx: &Vec<Transaction>, block_id: u64) -> TxVerifyResult {
    let mut prev_time = time;
    let length = tx.len();
    let mut cur_tx: &Transaction;
    let mut all_tx_hashes = "".to_string();
    let mut result = TxVerifyResult{time: "".to_string(), transaction_hashes: "".to_string(), validity: false};
    // iterate over all transactions
    for i in 0..length {
        cur_tx = &tx[i];
        // 1.ordinary transaction cases
        if i < length-1 {
            if !validate_normal_transaction(cur_tx, &prev_time) {
                return result;
            }
        }else {
            // 2.reward transaction case
            if !validate_reward_transaction(cur_tx, &prev_time, block_id) {
                return result;
            }
        }
        all_tx_hashes.push_str("|");
        all_tx_hashes.push_str(&cur_tx.hash.clone().unwrap());
        prev_time = &cur_tx.time;
    }
    // For blocks already created, all transactions passed, return success, but remember to check if there is a one and only reward tx
    result.time = prev_time.clone();
    result.transaction_hashes = all_tx_hashes;
    result.validity = true;
    return result;
}

// validate all_tx in one single block, including new_tx
// return: time: String, transaction_hashes: String, validity: bool
fn validate_and_complete_all_new_tx(time: &String, tx: & mut Vec<Transaction>) -> TxVerifyResult {
    // let last_block_time = time;
    let mut prev_time = time;
    // let length = tx.len();
    // let mut cur_tx: & mut Transaction;
    let mut all_tx_hashes = "".to_string();
    let mut result = TxVerifyResult{time: "".to_string(), transaction_hashes: "".to_string(), validity: false};
    // iterate over all transactions
    for cur_tx in tx.iter_mut() {
        // 1.ordinary transaction cases
        if cur_tx.send.is_some() {
            if !validate_normal_transaction(cur_tx, prev_time) {
                return result;
            }
        }else {
            // 2.incomplete transaction case
            if !validate_incomplete_transaction(cur_tx, prev_time) {
                return result;
            }
            complete_incomplete_transaction(cur_tx);
        }
        all_tx_hashes.push_str("|");
        all_tx_hashes.push_str(&cur_tx.hash.clone().unwrap());
        prev_time = &cur_tx.time;
    }
    result.time = prev_time.clone();
    result.transaction_hashes = all_tx_hashes;
    result.validity = true;
    return result;
}


fn decode_reader(bytes: Vec<u8>) -> io::Result<String> {
    let mut writer = Vec::new();
    let mut z = ZlibDecoder::new(writer);
    z.write_all(&bytes[..])?;
    writer = z.finish()?;
    let return_string = String::from_utf8(writer).expect("String parsing error");
    Ok(return_string)
}

fn compress_string(input: &String) -> Result<Vec<u8>, io::Error> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(input.as_bytes())?;
    encoder.finish()
}

fn validate_normal_transaction( tx: &Transaction, prev_time: &String) -> bool {
    let is_field_valid = tx.sig.is_some() && tx.fee.is_some() && tx.send.is_some() && tx.hash.is_some();
    let is_prev_time_less_than_cur_time = is_less_than(prev_time, &tx.time);
    let cc_hash = tx.hash.clone().unwrap();
    let is_hash_valid =  calculate_transaction_hash(tx).eq(&cc_hash);
    let is_signature_valid = verify_signature(tx, &cc_hash);
    //  I THINK IF YOU SEPARATE THEM UP WILL MAKE THIS FUNC MORE EFFICIENCY USING SHORTCUT
    if !is_field_valid || !is_prev_time_less_than_cur_time 
        || !is_hash_valid || !is_signature_valid{
        return false;
    }
    return true;
}

fn validate_reward_transaction(tx: &Transaction, prev_time: &String, block_id: u64) -> bool {
    let is_field_valid = tx.sig.is_none() && tx.fee.is_none() && tx.send.is_none() && tx.hash.is_some();
    let is_amt_invalid = AMT >> (block_id/2) != tx.amt;
    let is_prev_time_less_than_cur_time = is_less_than(prev_time, &tx.time);
    let is_hash_valid = calculate_transaction_hash(tx) == tx.hash.clone().unwrap();
    if !is_field_valid || is_amt_invalid || !is_prev_time_less_than_cur_time || !is_hash_valid{
        return false;
    }
    return true;
}

fn validate_incomplete_transaction(tx: &Transaction, prev_time: &String) -> bool {
    if !(tx.sig.is_none() && tx.fee.is_none() && tx.send.is_none() && tx.hash.is_none()) {
        return false;
    }
    if !is_less_than(prev_time, &tx.time) {
        return false;
    }
    return true;
}

fn complete_incomplete_transaction(tx: &mut Transaction) {
    tx.send = Option::from(E);
    tx.fee = Option::from(0);
    let tx_hash: String = calculate_transaction_hash(tx);
    tx.hash = Option::from(tx_hash.clone());
    let tx_hash_u64 = u64::from_str_radix(&tx_hash, 16).unwrap();
    let tx_sig: BigInt = rsa(tx_hash_u64, D, N);
    let tx_sig_u64: u64 = tx_sig.to_u64().unwrap();
    tx.sig = Option::from(tx_sig_u64);
}

fn add_reward_transaction(tx: & mut Vec<Transaction>, prev_time: &String, block_id: u64) -> String {
    let new_time = add_ten_minutes(&prev_time);
    let new_reward_amt = AMT >> (block_id/2);
    let mut reward_tx = Transaction{sig: Option::from(None), recv: E, amt: new_reward_amt, fee: Option::from(None), time: new_time, send: Option::from(None), hash: Option::from("".to_string())};
    let reward_tx_hash = calculate_transaction_hash(&reward_tx);
    reward_tx.hash = Option::from(reward_tx_hash.clone());
    tx.push(reward_tx);
    return reward_tx_hash;
}


fn calculate_transaction_hash(tx: &Transaction) -> String {
    let bar = "|".to_string();
    let mut hash_source = "".to_string();
    let send_str;
    if tx.send.is_none() {
        send_str = "".to_string();
    } else {
        send_str = tx.send.unwrap().to_string();
    }
    let fee_str;
    if tx.fee.is_none() {
        fee_str = "".to_string();
    } else {
        fee_str = tx.fee.unwrap().to_string();
    }
    hash_source = hash_source + &tx.time + &bar + &send_str + &bar + &tx.recv.to_string() + &bar + &tx.amt.to_string() + &bar + &fee_str;
    let sha256_hash = digest(hash_source);
    let cc_hash = sha256_hash.substring(0,8).to_string();
    return cc_hash;
}

fn verify_signature(tx: &Transaction, cc_hash: &String) -> bool {
    let cc_hash_bigint = BigInt::from_str_radix(cc_hash, 16).unwrap();
    let rsa_output = rsa(tx.sig.unwrap(), tx.send.unwrap(), N);
    return cc_hash_bigint.eq(&rsa_output);
}
fn rsa(message: u64, e: u64, n: u64) -> BigInt {
    let big_int_message = BigInt::from(message);
    let big_int_e = BigInt::from(e);
    let big_int_n = BigInt::from(n);
    let big_int_result = big_int_message.modpow(&big_int_e, &big_int_n);
    return big_int_result;
}

//Unit tests
#[cfg(test)]
mod tests {
    const TEST_STRING: &str = "eJyFk9tum0AQht9lr7mYw85heZWqigBDbClxqxg1lSK_ewcCBddJuldoF3a--f7hLXXH5nRO9be31Dw9PYy_58eXvvuVai-SRd3Ji1WpeR5TLbCsKo2n5z7VCcVJCDJsK1Xp2FyOcZhbMnOFdP1epZ8_XmNrOj0dUg1_XwLrilu2OBibl8d-nG5N1-qG6HJ6TDUKcRCwBVepVkpGViRAp9gb-oCiDL4AI6lnL1nvgKc7TAiLaVS-9OfDvx0vfB1k46Y9zEjvNZUwxx1MbOU_ZhCA0OMGl4KbmaYVHfDgOzMEtLjBzQ2IDAeUvRv4WE60D0ScobCtbgqIFQgHBVc35kiyMGc3LYT3xIWyUg4g2dTcaF7z1VZx8H4GmimoCDmia5kieKdAKOY5JiSMrRiwIDgbSfF7BlSPtDFagq_jabXDFmHYxXPb9lyH5JN4FBhIVQ2Ut3haMAn1sosH13Rolw65N-w36TTzN-f-9WHbg2m45639NEfpQgaCRKsqFVcjN2bgxRQL69IEZ1K2fCdLWZkMgxf168BwaLN3hjtZH_zln9bJYjm7ksI0uNc_g4cQeA==";
    use super::*;

    fn body_is_invalid(response_str: &String) -> bool {
       return response_str.substring(27, 34).to_string() == "INVALID";
    }

    fn validate_and_parse_response(response_str: &String) -> Result<Request, serde_json::Error> {
        // make sure it starts with CCprojectFor3,851725245278\n
        assert_eq!(response_str.substring(0, 27), "CCprojectFor3,851725245278\n");
        let encoded_json = response_str.substring(27, response_str.len()).to_string();
        let bytes_url = engine::GeneralPurpose::new(&alphabet::URL_SAFE, Default::default()).decode(encoded_json).unwrap();
        let json_str = decode_reader(bytes_url).unwrap();

        let json_result: Result<Request, serde_json::Error> = serde_json::from_str(&*json_str);
        return json_result;
    }
    #[test]
    fn test_valid_json() {
        let response_str = validate_encoded_json(&TEST_STRING.to_string());
        let json_result = validate_and_parse_response(&response_str);
        assert!(!json_result.is_err());
    }

    #[test]
    fn test_invalid_json() {
        let response_str = validate_encoded_json(&"eJyFk9tum0AQht9lr7mYw85heZWqigBDbClxqxg1lSK_ewcCBddJuldoF3a--f7hLXXH5nRO9be31Dw9PYy_58eXvvuVai-SRd3Ji1WpeR5TLbCsKo2n5z7VCcVJCDJsK1Xp2FyOcZhbMnOFdP1epZ8_XmNrOj0dUg1_XwLrilu2OBibl8d-nG5N1-qG6HJ6TDUKcRCwBVepVkpGViRAp9gb-oCiDL4AI6lnL1nvgKc7TAiLaVS-9OfDvx0vfB1k46Y9zEjvNZUwxx1MbOU_ZhCA0OMGl4KbmaYVHfDgOzMEtLjBzQ2IDAeUvRv4WE60D0ScobCtbgqIFQgHBVc35kiyMGc3LYT3xIWyUg4g2dTcaF7z1VZx8H4GmimoCDmia5kieKdAKOY5JiSMrRiwIDgbSfF7BlSPtDFagq_jabXDFmHYxXPb9lyH5JN4FBhIVQ2Ut3haMAn1sosH13Rol".to_string());
        assert!(body_is_invalid(&response_str.to_string()));
    }
}