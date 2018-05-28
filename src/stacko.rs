use datetime::{Instant, LocalDateTime};
use reqwest::get as http_get;
use serde_json::{self, Value as JsonValue};


#[derive(Debug)]
pub struct StackOverflowLink {
    pub title: String,
    pub url: String,
    pub last_update_time: LocalDateTime,
}

impl StackOverflowLink {
    pub fn get(question_id: u32) -> Self {
        let url = format!("https://api.stackexchange.com/2.2/questions/{}?site=stackoverflow", question_id);
        info!("Making API call to {:?}", url);
        let mut resp = http_get(&url).unwrap();
        assert!(resp.status().is_success());
        let text = resp.text().unwrap();

        let v: JsonValue = serde_json::from_str(&text).unwrap();

        let title = v["items"][0]["title"].as_str().unwrap().to_owned();
        let url   = v["items"][0]["link"].as_str().unwrap().to_owned();
        let last_update_time = LocalDateTime::at(v["items"][0]["last_activity_date"].as_i64().unwrap());
        StackOverflowLink { title, url, last_update_time }
    }

    pub fn is_recent(&self, now: Instant) -> bool {
        now.seconds() - self.last_update_time.to_instant().seconds() < 9999999
    }
}


/*
{
  "items": [
    {
      "tags": [
        "coding-style"
      ],
      "is_answered": true,
      "view_count": 221857,
      "closed_date": 1310000015,
      "accepted_answer_id": 218129,
      "answer_count": 112,
      "community_owned_date": 1224509768,
      "score": 173,
      "locked_date": 1317776078,
      "last_activity_date": 1326920053,
      "creation_date": 1224502695,
      "last_edit_date": 1495540499,
      "question_id": 218123,
      "link": "https://stackoverflow.com/questions/218123/what-was-the-strangest-coding-standard-rule-that-you-were-forced-to-follow",
      "closed_reason": "not constructive",
      "title": "What was the strangest coding standard rule that you were forced to follow?"
    }
  ],
  "has_more": false,
  "quota_max": 10000,
  "quota_remaining": 9994
}*/
