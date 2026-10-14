use std::fs::File;
use reqwest::header::{HeaderValue, RANGE};

const CHUNK_SIZE: u64 = 10240;
struct PartialRangeIter {
    start: u64,
    end: u64,
    buffer_size: u64,
}

impl PartialRangeIter {
    pub fn new(start: u64, end: u64, buffer_size: u64) -> Result<Self, anyhow::Error> {

        if buffer_size == 0 {
            return Err(anyhow::anyhow!("buffer size cannot be zero"))
        }

        Ok ( 
            Self {
                start,
                end,
                buffer_size
            }
        )
    }
}

impl Iterator for PartialRangeIter {
    
    type Item = HeaderValue;
    fn next(&mut self) -> Option<Self::Item> {
        
        if self.start > self.end {
            return None
        }
        let prev_start = self.start;
        self.start = self.start + std::cmp::min(self.buffer_size, self.end - self.start + 1);

        let header_value = HeaderValue::from_str(
                &format!("bytes={}-{}", prev_start, self.start - 1)
            ).unwrap(); 

        Some( header_value )
    }
}

pub async fn get_object(
    key: &str,
    presigned_url: &str,
    length: i64,
) -> anyhow::Result<()> {

    
    let client = reqwest::Client::new();
    
    let length = length as u64;

    let _output_file = File::create(key);
    for range in PartialRangeIter::new(0, length-1, CHUNK_SIZE)? {

        let response = client.get(presigned_url)
            .header(RANGE, range)
            .send()
            .await?;

        leptos::logging::log!("{}", response.text().await.unwrap());
    }

    Ok(())
}