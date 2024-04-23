// use crate::Base64Format;
use anyhow::Result;
use base64::engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD};
use base64::engine::Engine;
use std::io::Read;

use crate::cli::base64::Base64Format;

pub fn process_encode(reader: &mut dyn Read, format: Base64Format) -> Result<String> {
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    let encoded = match format {
        Base64Format::Standard => STANDARD.encode(&buf),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(&buf),
    };
    Ok(encoded)
}

pub fn process_decode(reader: &mut dyn Read, format: Base64Format) -> Result<String> {
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    // Remove trailing newline
    let buf = buf.trim();
    let decoded = match format {
        Base64Format::Standard => STANDARD.decode(buf)?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buf)?,
    };
    Ok(String::from_utf8(decoded)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    // 测试 process_encode 函数
    #[test]
    fn test_process_encode() {
        let mut reader = std::io::Cursor::new(b"hello".to_vec());
        let ret = process_encode(&mut reader, Base64Format::Standard).unwrap();
        assert_eq!(ret, "aGVsbG8=");
    }

    // 测试 process_decode 函数
    #[test]
    fn test_process_decode() {
        let mut reader = std::io::Cursor::new(b"aGVsbG8=".to_vec());
        let ret = process_decode(&mut reader, Base64Format::Standard).unwrap();
        assert_eq!(ret, "hello");
    }

    // Test encode and decode
    #[test]
    fn test_process_encode_and_decode() {
        let input = "README.md";

        // Read file content
        let mut buf = String::new();
        let mut file = std::fs::File::open(input).unwrap();
        file.read_to_string(&mut buf).unwrap();

        let mut reader = crate::utils::get_reader(input).unwrap();
        let res = process_encode(&mut reader, Base64Format::Standard).unwrap();

        let mut reader = std::io::Cursor::new(res.as_bytes().to_vec());
        let decode_res = process_decode(&mut reader, Base64Format::Standard).unwrap();

        assert_eq!(buf, decode_res);
    }
}
