extern crate github_v3;
extern crate iron;
extern crate time;
extern crate router;
extern crate rustc_serialize;

mod listening;
mod sending;
mod state;

use iron::prelude::*;
use iron::status;
use router::Router;
use std::env::args;
use std::path::PathBuf;
use iron::headers::{parsing, Header, HeaderFormatter, HeaderFormat};
use iron::error::HttpError;

use std::io::Read;

use std::fmt;
use std::ascii::AsciiExt;
use std::convert::From;

use github_v3::types::{GitErr};
use github_v3::event_types::{
  IssueCommentEvent,
  IssueCommentEventType,
  PullRequestEvent,
  PullRequestEventType,
  PullRequestReviewCommentEventType,
  PullRequestReviewCommentEvent,
  PushEvent,
  Pusher,
  PushCommit,
};

use rustc_serialize::{json};
use rustc_serialize::json::DecoderError;

fn handle_root(req: &mut Request) -> IronResult<Response> {
  Ok(Response::with((status::Ok, "cheapassbox.com rust + iron & soon catalyst")))
}

#[derive(Clone, PartialEq, Debug)]
pub struct GithubEventHeader {
  pub event_name: String
}

impl Header for GithubEventHeader {
  fn header_name() -> &'static str {
      "X-Github-Event"
  }
  fn parse_header(raw: &[Vec<u8>]) -> Result<GithubEventHeader, HttpError> {
    parsing::from_one_raw_str(raw).and_then(|s: String| {
      Ok(GithubEventHeader{event_name: s.to_ascii_lowercase()})
    })
  }

}

impl HeaderFormat for GithubEventHeader {
  fn fmt_header(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.write_str(&self.event_name)
  }
}

fn handle_webhooks(req: &mut Request) -> IronResult<Response> {
  println!("webhook hit {:?}", req);
  let mut payload = String::new();
  req.body.read_to_string(&mut payload).unwrap();
  println!("full payload {}", payload);

  let event_header = req.headers.get::<GithubEventHeader>();
  match event_header {
    Some(header) => {
      match header.event_name.as_ref() {
        "commit_comment" => {
          println!("got commit_comment");
          Ok(Response::with((status::Accepted, "{\"body\":\"ack\"}")))
        },
        "issue_comment" => {
          // Works in first test
          println!("got issue_comment");
          let payload: Result<IssueCommentEvent, DecoderError> = json::decode(&payload);
          println!("decoded {:?}", payload);
          Ok(Response::with((status::Accepted, "{\"body\":\"ack\"}")))
        },
        "issues" => {
          println!("got issues");
          Ok(Response::with((status::Accepted, "{\"body\":\"ack\"}")))
        },
        "pull_request_review_comment" => {
          // Works in first test
          println!("got pull_request_review_comment");
          let payload: Result<PullRequestReviewCommentEvent, DecoderError> = json::decode(&payload);
          println!("decoded {:?}", payload);
          Ok(Response::with((status::Accepted, "{\"body\":\"ack\"}")))
        },
        "pull_request" => {
          // Works in first test
          println!("got pull_request");
          let payload: Result<PullRequestEvent, DecoderError> = json::decode(&payload);
          println!("decoded {:?}", payload);

          Ok(Response::with((status::Accepted, "{\"body\":\"ack\"}")))
        },
        "push" => {
          // did not work in first test, there was a null 
          println!("got push");
          let payload: Result<PushEvent, DecoderError> = json::decode(&payload);
          println!("decoded {:?}", payload);
          Ok(Response::with((status::Accepted, "{\"body\":\"ack\"}")))
        },
        e @ _ => {
          println!("Unknown event header {:?}", e);
          Ok(Response::with((status::Accepted, "{\"body\":\"unknown event header\"}")))
        }
      }
    },
    None => {
      println!("No event header");
      Ok(Response::with((status::Accepted, "{\"body\":\"no event header\"}")))
    }
  }
}


fn main() {
  let token = std::env::var("CATALYST_GITHUB_OAUTH_TOKEN").unwrap();
  let repo_owner = std::env::var("CATALYST_REPO_OWNER").unwrap();
  let repo_name = std::env::var("CATALYST_REPO_NAME").unwrap();

  let mut router = Router::new();

  router.get("/", handle_root);
  router.post("/github_webhooks", handle_webhooks);

  //listening::start_listener(token, repo_owner, repo_name)
  Iron::new(router).http("0.0.0.0:8080").unwrap();
}
