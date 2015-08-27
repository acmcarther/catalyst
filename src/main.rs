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
  PushCommit,
};

use rustc_serialize::{json};
use rustc_serialize::json::DecoderError;
use std::sync::mpsc::{channel, Sender};

use iron::{BeforeMiddleware, AfterMiddleware, typemap};

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

enum EventType {
  IssueComment,
  PullRequestReviewComment,
  PullRequest,
  Push,
  UnknownEvent,
}

struct Deserialize;

impl BeforeMiddleware for Deserialize {
  fn before(&self, req: &mut Request) -> IronResult<()> {
    let mut payload = String::new();
    req.body.read_to_string(&mut payload).unwrap();

    let event_header = req.headers.get::<GithubEventHeader>();
    event_header.map(|header| {
      match header.event_name.as_ref() {
        "issue_comment" => {
          req.extensions.insert::<EventType>(EventType::IssueComment);
          req.extensions.insert::<IssueCommentEvent>(json::decode(&payload).unwrap());
        },
        "pull_request_review_comment" => {
          req.extensions.insert::<EventType>(EventType::PullRequestReviewComment);
          req.extensions.insert::<PullRequestReviewCommentEvent>(json::decode(&payload).unwrap());
        },
        "pull_request" => {
          req.extensions.insert::<EventType>(EventType::PullRequest);
          req.extensions.insert::<PullRequestEvent>(json::decode(&payload).unwrap());
        },
        "push" => {
          req.extensions.insert::<EventType>(EventType::Push);
          req.extensions.insert::<PushEvent>(json::decode(&payload).unwrap());
        },
        _ => req.extensions.insert::<EventType>(EventType::UnknownEvent)
      }
    });
    Ok()
  }
}


fn handle_webhooks(req: &mut Request) -> IronResult<Response> {
  let possible_event_type = *req.extensions.get::<EventType>();

  match possible_event_type {
    Some(EventType::IssueComment) => Ok(Response::with((status::Accepted, "{\"body\":\"Successful recv of issue comment\"}"))),
    Some(EventType::PullRequestReviewComment) => Ok(Response::with((status::Accepted, "{\"body\":\"Successful recv of pull request review comment\"}"))),
    Some(EventType::PullRequest) => Ok(Response::with((status::Accepted, "{\"body\":\"Successful recv of pull request\"}"))),
    Some(EventType::Push) => Ok(Response::with((status::Accepted, "{\"body\":\"Successful recv of push\"}"))),
    Some(EventType::UnknownEvent) => Ok(Response::with((status::Accepted, "{\"body\":\"Recv an unhandled event\"}"))),
    None => Ok(Response::with((status::Accepted, "{\"body\":\"No event header provided\"}")))
  }
}

struct DeliverActionables {
  issue_comment_tx: Sender<IssueCommentEvent>,
  pull_request_tx: Sender<PullRequestEvent>,
}

impl AfterMiddleware for DeliverActionables {
  fn after(&self, req: &mut Request, response: Response) -> IronResult<Response> {
    /*
    let possible_event_type = *req.extensions.get::<EventType>();
    match possible_event_type {
      Some(EventType::IssueComment) => {
        let possible_payload = *req.extensions.get::<IssueCommentEvent>();
        possible_payload.map(|payload: IssueCommentEvent| self.issue_comment_tx.send(5));

      },
      Some(EventType::PullRequestReviewComment) => {
        let possible_payload = *req.extensions.get::<PullRequestEvent>();
        possible_payload.map(|payload: PullRequestEvent| self.pull_request_event.send(5));
      },
      _ => ()
    }
    */
    Ok(response)
  }
}

fn main() {
  let token = std::env::var("CATALYST_GITHUB_OAUTH_TOKEN").unwrap();
  let repo_owner = std::env::var("CATALYST_REPO_OWNER").unwrap();
  let repo_name = std::env::var("CATALYST_REPO_NAME").unwrap();

  router.get("/", handle_root);
  router.post("/github_webhooks", webhook_chain);

  //listening::start_listener(token, repo_owner, repo_name)

  Iron::new(router).http("0.0.0.0:8080").unwrap();
}
