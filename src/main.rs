extern crate github_v3;
extern crate time;
extern crate rustc_serialize;
#[macro_use]
extern crate hyper;

mod listening;
mod sending;
mod state;


use std::io::Read;

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

use hyper::Server;
use hyper::server::{
  Request,
  Response
};
use hyper::status::StatusCode;
use hyper::net::Fresh;
use hyper::uri::RequestUri;

use hyper::header::{HeaderFormat, Header};
use hyper::error::Error as HttpError;
use hyper::header::parsing;

fn receive_payload(event_type: EventType, payload_str: String) {
  match event_type {
    EventType::IssueComment => {
      let result = json::decode::<IssueCommentEvent>(&payload_str).unwrap();
      println!("issue comment {:?}", result);
    },
    EventType::PullRequestReviewComment => {
      let result = json::decode::<PullRequestReviewCommentEvent>(&payload_str).unwrap();
      println!("pull request review comment {:?}", result);
    },
    EventType::PullRequest => {
      let result = json::decode::<PullRequestEvent>(&payload_str).unwrap();
      println!("pull request {:?}", result);
    },
    EventType::Push => {
      let result = json::decode::<PushEvent>(&payload_str).unwrap();
      println!("push {:?}", result);
    },
    EventType::UnknownEvent => {
      println!("unknown {:?}", payload_str);
    },
  }
}

fn handle_general(req: Request, mut res: Response<Fresh>) {
  match req.method {
    hyper::Post => {
      handle_posts(req, res)
    }
    hyper::Get => {
      handle_gets(req, res)
    },
    _ => *res.status_mut() = StatusCode::MethodNotAllowed
  }
}

fn handle_posts(req: Request, mut res: Response<Fresh>) {
  match req.uri.clone() {
    RequestUri::AbsolutePath(path) => {
      match path.as_ref() {
        "/github_webhooks" => handle_webhooks(req, res),
        _ => *res.status_mut() = StatusCode::NotFound
      }
    },
    _ => *res.status_mut() = StatusCode::NotFound
  }
}

fn handle_gets(req: Request, mut res: Response<Fresh>) {
  match req.uri.clone() {
    RequestUri::AbsolutePath(path) => {
      match path.as_ref() {
        "/" => handle_root(req, res),
        "/github_webhooks" => *res.status_mut() = StatusCode::MethodNotAllowed,
        _ => *res.status_mut() = StatusCode::NotFound
      }
    },
    _ => *res.status_mut() = StatusCode::NotFound
  }
}

fn handle_root(_: Request, mut res: Response<Fresh>) {
  *res.status_mut() = StatusCode::Ok;
  res.send(b"cheapassbox.com rust + hyper & soon catalyst").unwrap();
}

fn handle_webhooks(mut req: Request, mut res: Response<Fresh>) {

  let clone_headers = req.headers.clone();
  let github_header = clone_headers.get::<XGithubEvent>();
  match github_header {
    Some(event) => {
      let event_type =
        match event.as_ref() {
          "issue_comment" => EventType::IssueComment,
          "pull_request_review_comment" => EventType::PullRequestReviewComment,
          "pull_request" => EventType::PullRequest,
          "push" => EventType::Push,
          _ => EventType::UnknownEvent,
        };
      let mut payload_str = String::new();
      req.read_to_string(&mut payload_str).unwrap();
      receive_payload(event_type, payload_str);
      *res.status_mut() = StatusCode::Accepted;
      res.send(b"{\"body\": \"hello\"}").unwrap();
    },
    None => {
      *res.status_mut() = StatusCode::UnprocessableEntity;
      res.send(b"{\"body\": \"No status code present\"}").unwrap();
    }
  }
}

header! { (XGithubEvent, "X-Github-Event") => [String] }

enum EventType {
  IssueComment,
  PullRequestReviewComment,
  PullRequest,
  Push,
  UnknownEvent,
}

fn main() {
  let token = std::env::var("CATALYST_GITHUB_OAUTH_TOKEN").unwrap();
  let repo_owner = std::env::var("CATALYST_REPO_OWNER").unwrap();
  let repo_name = std::env::var("CATALYST_REPO_NAME").unwrap();

  Server::http("0.0.0.0:8080").unwrap().handle(handle_general).unwrap();
}
