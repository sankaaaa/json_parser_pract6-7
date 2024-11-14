use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use serde::{Serializer, Deserializer};
use std::{time::Duration};
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    name: String,
    email: String,
    b_day: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PublicTariff {
    id: u32,
    price: u32,
    #[serde(with = "humantime_serde")]
    duration: Duration,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PrivateTariff {
    client_price: u32,
    #[serde(with = "humantime_serde")]
    duration: Duration,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Stream {
    user_id: Uuid,
    is_private: bool,
    settings: i32,
    shard_url: Url,
    public_tariff: PublicTariff,
    private_tariff: PrivateTariff,
}

#[derive(Debug, Serialize, Deserialize)]
struct Gift {
    id: u32,
    price: u32,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Debug {
    duration: String,
    at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Request {
    #[serde(rename = "type")]
    req_type: HttpStatus,
    stream: Stream,
    gifts: Vec<Gift>,
    debug: Debug,
}

#[derive(Debug, Serialize, Deserialize)]
struct Event {
    name: String,
    #[serde(
        serialize_with = "serialize_data", 
        deserialize_with = "deserialize_data"
    )]
    date: String,
}

fn serialize_data<S: Serializer>(date: &str, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&format!("Date: {}", date))
}

fn deserialize_data<'de, D: Deserializer<'de>>(deserializer: D) -> Result<String, D::Error> {
    let data: &str = Deserialize::deserialize(deserializer)?;
    Ok(data.replace("Date: ", ""))
}

#[derive(Debug, Serialize, Deserialize)]
enum HttpStatus {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "bad_request")]
    BadRequest,
    #[serde(rename = "unprocessable_entity")]
    UnprocessableEntity,
}

fn main() {
    let event = Event {
        name: "Event 1".to_string(),
        date: "2024-12-12".to_string(),
    };
    let json = serde_json::to_string(&event).unwrap();
    println!("\nJSON:\n{}", json);

    let deserialized_event: Event = serde_json::from_str(&json).unwrap();
    println!("\nDeserialized JSON:\n{:?}", deserialized_event);

     // let mut file = File::open("request.json").unwrap();

    // let mut file_str: String = String::new();
    // file.read_to_string(&mut file_str).unwrap();
    // // print!("{file_str}");

    // let request: Request = serde_json::from_str(&file_str).unwrap();
    // // print!("{:?}", request)

    // let yaml_str = serde_yaml::to_string(&request).unwrap();
    // print!("{yaml_str}\n");

    // let toml_str = toml::to_string(&request).unwrap();
    // print!("{toml_str}\n");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let event = Event {
            name: "Event 1".to_string(),
            date: "2024-12-12".to_string(),
        };
        let json = serde_json::to_string(&event).unwrap();
        let deserialized_event: Event = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized_event.name, "Event 1");
        assert_eq!(deserialized_event.date, "2024-12-12");
    }
}
