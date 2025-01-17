use regex::Regex;
use serde_json::Value;
use sha2::{Digest, Sha512};




pub fn control_body(list_needed: Vec<&str>, body: &Value) -> bool {
    for item in list_needed {
        if !body.get(item).is_some() {
            return false;
        }
    }
    return true;
}

// String manipulation part
pub fn sha512_string(input: &str) -> String {
    let mut hasher = Sha512::new();
    hasher.update(input);
    let result = hasher.finalize();
    let hash_string = result.iter().map(|byte| format!("{:02x}", byte)).collect::<String>();
    hash_string
}

pub fn extract_string_from_obj_value(content:Option<&Value>) -> String {
    return match content {
        Some(extract_value) => {
            match extract_value.as_str() {
                Some(v) => v.to_owned(), // Convert &str to String
                None => { String::new() }
            }
        }
        None => { String::new() }
    };
}




/***
 *     ____  ____  ___  ____  _  _ 
 *    (  _ \( ___)/ __)( ___)( \/ )
 *     )   / )__)( (_-. )__)  )  ( 
 *    (_)\_)(____)\___/(____)(_/\_)
 */
pub fn is_valid_email(email: &str) -> bool {
    let re = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,10}$").unwrap();
    re.is_match(email)
}

pub fn is_valid_username(username: &str) -> bool {
    let re = Regex::new(r"^[a-zA-Z0-9._-]{3,25}$").unwrap();
    re.is_match(username)
}

pub fn is_valid_dpusername(dpusername: &str) -> bool {
    let re = Regex::new(r"^[a-zA-Z0-9\ ._-]{3,25}$").unwrap();
    re.is_match(dpusername)
}

pub fn is_valid_text(text: &str) -> bool {
    let re = Regex::new(r"^[a-zA-Z0-9<>'\ \/\:.,_-]{3,200}$").unwrap();
    re.is_match(text)
}

pub fn is_uuid_v4(input: &str) -> bool {
    let re = Regex::new(r"^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$").unwrap();
    re.is_match(input)
}

pub fn is_valid_url(input: &str) -> bool {
    let re = Regex::new(r"^(https?://)?(?:[\w-]+\.)?[\w-]+\.[a-zA-Z]{2,}(?:/[\w/]{1,100})?$").unwrap();
    re.is_match(input)
}

pub fn is_valid_url_local(input: &str) -> bool {
    let re = Regex::new(r"^(?:/[\w\./]{1,100})$").unwrap();
    re.is_match(input)
}

pub fn is_valid_number(input: &str) -> bool {
    let re = Regex::new(r"^[0-9]{25}$").unwrap();
    re.is_match(input)
}

pub fn is_valid_sha512(input: &str) -> bool {
    let re = Regex::new(r"^[0-9a-f]{128}$").unwrap();
    re.is_match(input)
}