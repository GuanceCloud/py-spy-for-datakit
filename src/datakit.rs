use std::collections::HashMap;
use chrono::Utc;
use chrono::SecondsFormat;
use chrono::DateTime;
use reqwest::blocking::Client;
use reqwest::blocking::multipart::{Form, Part};
use mime;
use serde_derive::Serialize;

pub const VERSION:&str = "0.1";
pub const PROFILING_ENDPOINT_V1:&str = "/profiling/v1/input";

#[derive(Serialize)]
pub struct Event {
    pub attachments: Vec<String>,
    pub format: Option<String>,
    pub tags_profiler: String,
    pub start: Option<String>,
    pub end: Option<String>,
    pub family: String,
    pub version: String,
}

pub struct Sample {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub payload: Vec<u8>,
}

fn join_tags(tags: &HashMap<String, String>) -> String {
    let mut tags_vec:Vec<String> = Vec::with_capacity(tags.len());
    for (k, v) in tags.iter() {
        tags_vec.push(format!("{}:{}", k, v))
    }
    return tags_vec.join(",")
}

#[test]
fn test_join_tags() {

    let tags = HashMap::from([("env".to_string(), "test".to_string()),
        ("process_id".to_string(), "10238".to_string()),
        ("host".to_string(), "zydeMacBook-Air-3.local".to_string()),
        ("service".to_string(), "zy-profiling-test".to_string()),
        ("version".to_string(), "v1.2".to_string()),
        ("runtime-id".to_string(), "4fb083fa-31e6-4cb8-a506-f4e36b9657ec".to_string()),
        ("runtime_version".to_string(), "11.0.15".to_string()),
        ("profiler_version".to_string(), "0.108.1~5d2b67be17".to_string()),
        ("language".to_string(), "jvm".to_string()),
        ("library_version".to_string(), "0.108.1~5d2b67be17".to_string())
    ]);

    println!("{}", join_tags(&tags))

}

impl Event {
    pub fn new(tags: &HashMap<String, String>) -> Event {

        Event{
            attachments: vec![String::from("main.txt")],
            format: Some(String::from("rawflamegraph")),
            tags_profiler: join_tags(tags),
            start: None,
            end: None,
            family: "python".to_string(),
            version: VERSION.to_string(),
        }

    }
}

pub fn send_to_datakit(client:&Client, endpoint: &str, mut event: Event, sample: Sample) -> Result<String, anyhow::Error> {

    event.start = Some(sample.start.to_rfc3339_opts(SecondsFormat::Micros, true));
    event.end = Some(sample.end.to_rfc3339_opts(SecondsFormat::Micros, true));

    let resp = client.post(endpoint)
        .multipart(Form::new()
            .part("event", Part::text(serde_json::to_string(&event)?)
                .file_name("event.json")
                .mime_str(mime::APPLICATION_OCTET_STREAM.as_ref())?)
            .part("main", Part::bytes(sample.payload)
                .file_name("main.txt")
                .mime_str(mime::APPLICATION_OCTET_STREAM.as_ref())?
            )
        )
        .send()?;

    if resp.status().as_u16() / 100 == 2 {
        return Ok(resp.text()?);
    }

    Err(format_err!("http response status: {}", resp.status().to_string()))
}