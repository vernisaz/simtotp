/* Generating TOTP in Rust involves implementing the algorithm described in RFC 6238. This typically uses the HMAC-SHA1 cryptographic function along with a time component.
Below is an example of Rust code for generating a TOTP.
*/
// src/main.rs
extern crate base32;
extern crate simweb;
extern crate simjson;
extern crate simcfg;
mod sha1;
mod hmac;
use sha1::Sha1;
use std::{time::{SystemTime, UNIX_EPOCH}, fmt, convert::TryInto, error::Error};
use hmac::hmac;
use simweb::{WebPage,json_encode};
use simjson::{JsonData::{self}};
   
const VERSION: &str = env!("VERSION");

#[derive(Debug)]
pub struct TOTPError {
    cause: String,
}
impl fmt::Display for TOTPError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Err: {}", self.cause)
    }
}
impl std::error::Error for TOTPError {}

/// Generates a TOTP code.
///
/// # Arguments
///
/// * `secret` - The secret key as a byte slice.
/// * `digits` - The number of digits for the TOTP code (e.g., 6 or 8).
/// * `step_seconds` - The time step in seconds (e.g., 30).
///
/// # Returns
///
/// An `Option<u32>` containing the TOTP code if successful, otherwise `None`.
pub fn generate_totp(secret: &[u8], digits: u32, step_seconds: u64) -> Option<u32> {
    let current_time_seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .ok()?
        .as_secs();

    let time_step = current_time_seconds / step_seconds;

    // Convert the time step to an 8-byte big-endian array.
    let time_bytes = time_step.to_be_bytes();

    let result = hmac(secret, &time_bytes, 64); 
    
    let code = hotp_from_hmac(&result, digits); 

    Some(code)
}

/// Extracts an HOTP code from an HMAC result.
fn hotp_from_hmac(hmac_result: &[u8], digits: u32) -> u32 {
    let offset = (hmac_result[19] & 0xf) as usize;
    let hmac_truncated = hmac_result[offset..offset + 4].to_vec();
    let otp = u32::from_be_bytes(hmac_truncated.try_into().unwrap()) & 0x7fff_ffff;

    let power_of_10 = 10u32.pow(digits);
    otp % power_of_10
}

use base32::Alphabet;
use std::{env, fs::{self, read_to_string}, path::{PathBuf}, io::{self,Write}, collections::HashMap,
    fmt::Write as fmtWrite,
};

struct Response<'a> {
    json: &'a str,
}

fn main() -> Result<(), Box<dyn Error>> {
   #[cfg(test)]
    {
    let test = hmac(b"key", b"The quick brown fox jumps over the lazy dog", 64);
    assert_eq!(simweb::to_hex(&test), "de7c9b85b8b78aa6bc8a7a36f70a90701c9db4d9")
    }
    let totp = std::env::current_exe();
    
    let mut home = PathBuf::new();
    let mut home_set = false;
    if let Ok(ws_exe) = totp &&
        let Some(current_path) = ws_exe.parent() {
        let home_file = current_path.join(".config");
        if let Ok(home_str) = read_to_string(&home_file) {
            home = PathBuf::from(home_str.trim());
            home_set = true;
        } else {
            eprintln! {"Misconfiguration: config home isn't set in .config in {:?} yet", &home_file};
            match simcfg::get_config_root() {
                Ok(config_dir) => {fs::write(&home_file, config_dir.display().to_string())?;
                    home = config_dir;
                    home_set = true
                }
                Err(err) => {
                    eprintln! {"{err:?}"}
                }
            }
        }
    }
    if !home_set {
        match std::env::home_dir() {
                Some(dir) => {
                    home = dir
                }
                None => {
                    eprintln!("Can't obtain config directory from env or file, exiting");
                    std::process::exit(1)
                }
        }
    }
    home.push(".simtotp");
    if !home.exists() {
        fs::create_dir_all(&home)?;
    }
    home.push("directory"); home.set_extension("db");
    if std::env::var("QUERY_STRING").is_err() { // run as CLI
        // CLI mode
        let args: Vec<String> = env::args().collect();
        if args.len() <= 3  {
            eprintln!("Simple TOTP v-{VERSION}");
            eprintln!("No program arguments from web or CLI");
            std::process::exit(1)
        }
        let query_str = format!("pass={}&op={}&name={}&account={}&secret={}",
            simweb::url_encode(&args[1]), args[2], if args.len() > 3 {simweb::url_encode(&args[3])} else {"".to_string()},
            if args.len() > 4 {simweb::url_encode(&args[4])} else {"".to_string()}, if args.len() > 5 {args[5].clone()} else {"".to_string()});
        eprintln!("{query_str}");
        unsafe {
            env::set_var("QUERY_STRING",query_str)
        }
    }

    let web = simweb::WebData::new();
    let mut password;
    match web.param("pass") {
        Some(pass) => { password = pass; }
        _ => {
            Response {
                json:r#"{"error":"no password"}"#,
            }.show();
            return Ok(())
        }
    }
    let mut namespaces = match read_db(&home, &password) {
        Ok(namespaces) => namespaces,
        Err(err) => {
            Response {
                json:&format!(r#"{{"error":"DB can't be read correctly {err}"}}"#),
            }.show();
            return Ok(())
        }
    };
    let mut json:&str = "{}";
    let mut update_db = false;
    let op = web.param("op").unwrap_or_default();
    let res;
    let code_str: String;
    match op.as_str() {
        "lsns" => { // list of namespaces
            res = format!("[{}]", namespaces.keys().map(|k| format!(r#""{}""#, json_encode(k))).collect::<Vec<_>>().join(","));
            json = &res
        }
        "lsac" => { // list of accounts in a namespace
            match web.param("name") {
                Some(ns) => {
                    let acns = namespaces.get(&ns);
                    if let Some(acns) = acns {
                        res = format!("[{}]", acns.keys().map(|k| format!(r#""{}""#, json_encode(k))).collect::<Vec<_>>().join(","));
                        json = &res
                    } else { json = r#"{"error":"no namespace"}"# }
                }
                _ => json = r#"{"error":"no namespace name"}"#,
            }
        }
        "gen" => { // generate TOTP code
            if let Some(name) = web.param("name") &&
                let Some(acn) = web.param("account") &&
                let Some(ns) = namespaces.get(&name) &&
                let Some(web_secret) = ns.get(&acn) {
                    let digits = 6;
                    let step = 30;
                    if let Some(secret) = base32::decode(Alphabet::Rfc4648 { padding: false }, web_secret) {
                        match generate_totp(&secret, digits, step) {
                            Some(code) => {
                                code_str = format!(r#"{{"code":"{:0>width$}"}}"#, code, width = digits as usize);
                                json = &code_str;
                                eprintln!("Current TOTP code: {:0>width$}", code, width = digits as usize);
                            }
                            None => {
                                json = r#"{"error":"Failed to generate TOTP code."}"#;
                                eprintln!("Failed to generate TOTP code.");
                            }
                        }
                    } else {
                        json = r#"{"error":"The secret isn't valid base32 value."}"#;
                    }
            } else {
                json = r#"{"error":"Insufficient info to generate TOTP code."}"#;
            }
        }
        "adac" => { // add an account with a secret
            if let Some(name) = web.param("name") &&
                let Some(acn) = web.param("account") &&
                let Some(secret) = web.param("secret") {
                if let Some(ns) = namespaces.get_mut(&name) {
                    ns.insert(acn, secret);
                } else {
                    let mut ns = HashMap::new();
                    ns.insert(acn, secret);
                    namespaces.insert(name.clone(),ns);
                }
                update_db = true;
                json = r#"{"ok":true}"#;
            } else {
                 json = r#"{"error":"Insufficient info to add an account."}"#;
            }
        }
        "upse" => { // update a secret for an account
            if let Some(name) = web.param("name") &&
                let Some(acn) = web.param("account") &&
                let Some(secret) = web.param("secret") &&
                let Some(ns) = namespaces.get_mut(&name) &&
                !secret.is_empty() {
                    // TODO if no such account
                    ns.insert(acn, secret);
                    update_db = true;
                    json = r#"{"ok":true}"#;
            } else {
                json = r#"{"error":"Insufficient info to update the secret."}"#;
            }
        }
        "deac" => { // delete an account
            if let Some(name) = web.param("name") &&
                let Some(acn) = web.param("account") &&
                let Some(ns) = namespaces.get_mut(&name) &&
                ns.remove(&acn).is_some() {
                    update_db = true;
                    json = r#"{"ok":true}"#;
            } else {
                json = r#"{"error":"No such account."}"#;
            }
        }
        "dens" => { // delete a namespace
            let mut done = false;
            if let Some(name) = web.param("name") 
                && namespaces.remove(&name).is_some() {
                done = true;
                update_db = true;
                json = r#"{"ok":true}"#;
            }
            if !done {
                 json = r#"{"error":"No such namespace."}"#;
            }
        }
        "mons" => { // modify a namespace name
            let mut done = false;
            if let Some(name) = web.param("name") 
                && let Some(new_name) = web.param("newname")
                && let Some(ns) = namespaces.remove(&name) {
                    namespaces.insert(new_name, ns);
                    done = true;
                    update_db = true;
                    json = r#"{"ok":true}"#;
            }
            if !done {
                 json = r#"{"error":"No such namespace."}"#;
            }
        }
        "moac" => { // modify an account name
            let mut done = false;
            if let Some(name) = web.param("name") 
                && let Some(acn) = web.param("account") 
                && let Some(new_name) = web.param("newname")
                && let Some(ns) = namespaces.get_mut(&name) 
                && !new_name.is_empty() {
                let secret = ns.remove(&acn);
                if let Some(secret) = secret {
                    ns.insert(new_name, secret);
                    done = true;
                    update_db = true;
                    json = r#"{"ok":true}"#;
                }
            }
            if !done {
                 json = r#"{"error":"No update the account."}"#;
            }
        }
        "uppa" => { // update password
            if let Some(pass) = web.param("newpassword") {
                if !pass.is_empty() {
                    update_db = true;
                    password = pass;
                    json = r#"{"ok":true}"#;
                } else {
                    json = r#"{"error":"no new password"}"#;
                }
            }
        }
        "dndb" => { // download db
            if let Some(dn_password) = web.param("dnpassword") {
                let db = write_db(&dn_password, &namespaces);
                // Content-Lengt will be recalculated by CGI provider anyway
                print!("Content-Length: {}\r\nContent-Type: application/octet-stream\r\nContent-Disposition: attachment; filename=\"totp.db\"\r\n\r\n", db.len());
                io::stdout().write_all(&db[..])?
            }
            json = r#"{"error":"no no db password"}"#;
        }
         "updb" => { // upload db
            match web.param("upFile") {
                None => json = r#"{"error":"nothing was uploaded"}"#,
                Some(file) => {
                    let up_password = web.param("uppassword") .unwrap_or_default();
                    let up_file = PathBuf::from(&file);
                    match read_db(&up_file, &up_password) {
                        Ok(new_namespaces) => {
                            namespaces = new_namespaces;
                            let _ = fs::remove_file(up_file);
                            update_db = true;
                            json = r#"{"ok":true}"#;
                        }
                        Err(err) => {code_str = format!("Can't correctly read new DB {err}");
                            json = &code_str;},
                    }
                }
            }
        }
        "vers" => {code_str = format!(r#"{{"version":"v{VERSION}","ok":true}}"#);
            json = &code_str;},
        _ => { // op error
            json = r#"{"error":"unknown op"}"#;
        }
    }
    Response {
        json,
    }.show();
    if update_db {
        Ok(fs::write(&home,write_db(&password, &namespaces))?)
    } else {
        Ok(())
    }
}

impl simweb::WebPage for Response<'_> { 
    fn main_load(&self) -> Result<String, Box<dyn std::error::Error + 'static>> {
        Ok(self.json.to_string ())
    }
    fn content_type(&self) -> &str {
        "application/json"
    }
}

fn read_db<'a>(home: &'a PathBuf, password: &'a str) -> Result<HashMap<String, HashMap<String,String>>, TOTPError> {
    let mut res = HashMap::new();
    match fs::read(home) {
        Ok(mut data) => {
            let password = password.as_bytes();
            if !password.is_empty() {
                for i in 0..data.len() {
                    data[i] ^= password[i % password.len()]
                }
            }
            //eprintln!("{}", String::from_utf8_lossy(&data));
            let json_db = simjson::parse(&String::from_utf8_lossy(&data));
            match json_db {
                JsonData::Data(ns) => {
                    for (key, value) in ns.iter() {
                        if key.is_empty() { continue}
                        match value {
                            JsonData::Data(acn) => {
                                let mut a_res = HashMap::new();
                                for (a_key, a_value) in acn.iter() {
                                    if a_key.is_empty() { continue }
                                    if let JsonData::Text(secret) = a_value {
                                         a_res.insert(a_key.to_string(), secret.to_string());
                                     }
                                }
                                res.insert(key.to_string(), a_res);
                            }
                            _ => ()
                        }
                    }
                }
                JsonData::Err(reason) => return Err(TOTPError{cause:format!("corrupted DB, probably wrong password: {reason}")}),
                _ => return Err(TOTPError{cause:"unexpected data organization".to_string()})
            }
        }
         _ => eprintln!("new DB created")
    }
    Ok(res)
}

fn write_db(password: &str, db: &HashMap<String, HashMap<String,String>>) -> Vec<u8> {
    let mut res = String::from("{");
    for (key, value) in db.iter() {
        if key.is_empty() { continue }
        write!(res,r#""{key}":{{"#).unwrap();
        for (acn, secret) in value.iter() {
            if !acn.is_empty() {
                 write!(res,r#""{acn}":"{secret}","#).unwrap();
            }
        }
        // no json encodibg
        write!(res,r#""":""}},"#).unwrap();
    }
    write!(res,r#""":{{}} }}"#).unwrap();
    let password = password.as_bytes();
    let mut byte_vec: Vec<u8> = res.into_bytes();
    if !password.is_empty() {
        for i in 0..byte_vec.len() {
            byte_vec[i] ^= password[i % password.len()]
        } 
    }
    byte_vec
}

/*
Explanation
 * generate_totp function:
   * It first gets the current system time in seconds since the Unix epoch.
   * It calculates the current time step by dividing the current time by the step_seconds (e.g., 30).
   * The time step is converted into an 8-byte big-endian array, which is the message used in the HMAC calculation.
   * An HmacSha1 instance is created with the secret key.
   * The update method adds the time step to the HMAC context.
   * finalize computes the HMAC digest.
   * The digest is then passed to the hotp_from_hmac function to extract the final passcode.
 * hotp_from_hmac function:
   * This function implements the "dynamic truncation" part of the algorithm.
   * It takes the last 4 bits of the HMAC digest (at index 19) to determine an offset into the digest.
   * It then takes 4 bytes starting from that offset.
   * The most significant bit of the first byte is masked off to ensure the result is a positive integer.
   * These 4 bytes are combined into a single u32 value.
   * Finally, a modulo operation is performed with a power of 10 to get the desired number of digits.
This example provides a clear, self-contained implementation of the TOTP algorithm in Rust. 
For a more robust solution in a real-world application, you might consider using a dedicated library like otp-rs which handles more details,
such as base32 secret key decoding and error handling.
*/