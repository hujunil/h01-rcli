use anyhow::Result;

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    aud: String, // Optional. Audience
    exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    sub: String, // Optional. Subject (whom token refers to)
}

pub fn sign_jwt(sub: &str, aud: &str, exp: &str) -> anyhow::Result<String> {
    let now = chrono::Utc::now();
    let timestamp = now.timestamp();

    let claim = Claims {
        aud: aud.to_string(),
        sub: sub.to_string(),
        exp: (parse_exp(exp)? + timestamp) as usize,
    };

    encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret("hello".as_ref()),
    )
    .map_err(Into::into)
}

pub fn verify_jwt(token: &str) -> anyhow::Result<TokenData<Claims>> {
    // String::from("claims")
    let mut validation = Validation::default();
    validation.set_audience(&["tester", "developer", "leader"]);

    decode::<Claims>(
        token,
        &DecodingKey::from_secret("hello".as_ref()),
        &validation,
    )
    .map_err(Into::into)
}

fn parse_exp(exp: &str) -> Result<i64> {
    let re = regex::Regex::new(
        r"^(\d+)(day|d|month|M|year|y|week|w|hour|h|minute|m|second|s|millisecond|ms)$",
    )
    .unwrap();
    let caps = re.captures(exp).unwrap();
    let num = caps.get(1).unwrap().as_str();
    let unit = caps.get(2).unwrap().as_str();
    let duration = match unit {
        "day" | "d" => chrono::Duration::days(num.parse().unwrap()),
        "month" | "M" => chrono::Duration::days(num.parse::<i64>().unwrap() * 30),
        "year" | "y" => chrono::Duration::days(num.parse::<i64>().unwrap() * 365),
        "week" | "w" => chrono::Duration::weeks(num.parse().unwrap()),
        "hour" | "h" => chrono::Duration::hours(num.parse().unwrap()),
        "minute" | "m" => chrono::Duration::minutes(num.parse().unwrap()),
        "second" | "s" => chrono::Duration::seconds(num.parse().unwrap()),
        "millisecond" | "ms" => chrono::Duration::milliseconds(num.parse().unwrap()),
        _ => {
            return Err(anyhow::anyhow!("Invalid unit: {}", unit));
        }
    };
    Ok(duration.num_seconds())
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_parse_exp() {
        let size = super::parse_exp("1h").unwrap();
        assert_eq!(size, 3600);
        let size = super::parse_exp("1d").unwrap();
        assert_eq!(size, 86400);
        let size = super::parse_exp("1w").unwrap();
        assert_eq!(size, 604800);
    }
}
