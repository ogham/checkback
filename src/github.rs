use datetime::{Instant, LocalDateTime};
use reqwest::get as http_get;
use serde_json::{self, Value as JsonValue};


#[derive(Debug)]
pub struct GitHubLink {
    pub title: String,
    pub url: String,
    pub last_update_time: i64,
}

impl GitHubLink {
    pub fn get(user: &str, repo: &str, issue_id: u32) -> Self {
        use std::str::FromStr;

        let url = format!("https://api.github.com/repos/{}/{}/issues/{}", user, repo, issue_id);
        info!("Making API call to {:?}", url);

        let mut resp = http_get(&url).unwrap();
        assert!(resp.status().is_success());
        let text = resp.text().unwrap();

        let v: JsonValue = serde_json::from_str(&text).unwrap();

        let title = v["title"].as_str().unwrap().to_owned();
        let url   = v["url"].as_str().unwrap().to_owned();
        let iso   = v["updated_at"].as_str().unwrap();

        let last_update_time = LocalDateTime::from_str(iso).unwrap().to_instant().seconds();
        GitHubLink { title, url, last_update_time }
    }

    pub fn is_recent(&self, now: Instant) -> bool {
        now.seconds() - self.last_update_time < 9999999
    }
}

/*
[
  {
    "url": "https://api.github.com/repos/rust-lang/rust/issues/comments/330915766",
    "html_url": "https://github.com/rust-lang/rust/issues/44265#issuecomment-330915766",
    "issue_url": "https://api.github.com/repos/rust-lang/rust/issues/44265",
    "id": 330915766,
    "user": {
      "login": "nikomatsakis",
      "id": 155238,
      "avatar_url": "https://avatars0.githubusercontent.com/u/155238?v=4",
      "gravatar_id": "",
      "url": "https://api.github.com/users/nikomatsakis",
      "html_url": "https://github.com/nikomatsakis",
      "followers_url": "https://api.github.com/users/nikomatsakis/followers",
      "following_url": "https://api.github.com/users/nikomatsakis/following{/other_user}",
      "gists_url": "https://api.github.com/users/nikomatsakis/gists{/gist_id}",
      "starred_url": "https://api.github.com/users/nikomatsakis/starred{/owner}{/repo}",
      "subscriptions_url": "https://api.github.com/users/nikomatsakis/subscriptions",
      "organizations_url": "https://api.github.com/users/nikomatsakis/orgs",
      "repos_url": "https://api.github.com/users/nikomatsakis/repos",
      "events_url": "https://api.github.com/users/nikomatsakis/events{/privacy}",
      "received_events_url": "https://api.github.com/users/nikomatsakis/received_events",
      "type": "User",
      "site_admin": false
    },
    "created_at": "2017-09-20T17:00:23Z",
    "updated_at": "2017-09-21T14:23:54Z",
    "author_association": "CONTRIBUTOR",
    "body": "Here is a kind of implementation plan I will endeavor to keep updated.\r\n\r\n- [ ] Step one: add support into the AST and pretty-printing\r\n    - Probably the first step is to start *parsing* the new forms and to add support for them to the AST. \r\n    - See [this comment for more detailed thoughts here](https://github.com/rust-lang/rust/issues/44265#issuecomment-331172238). \r\n    - We should be able to write some parsing-only tests and also test the pretty-printer hir\r\n    - When we get to [HIR lowering](https://github.com/rust-lang/rust/blob/master/src/librustc/hir/lowering.rs), we can error out if any GAT are present\r\n    - We can also do the feature gate then\r\n- [ ] More to come"
  },
]*/
