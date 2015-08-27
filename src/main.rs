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
use iron::headers::{parsing, Header, HeaderFormat};
use iron::error::HttpError;

use std::io::Read;

use std::fmt;
use std::ascii::AsciiExt;
use std::sync::Mutex;

use github_v3::{GithubClient, IssueCommenter};

use github_v3::event_types::{
  IssueCommentEvent,
  PullRequestEvent,
  PullRequestReviewCommentEvent,
  PushEvent,
};

use rustc_serialize::{json};
use std::sync::mpsc::{channel, Sender, TryRecvError};

use iron::{BeforeMiddleware, AfterMiddleware, typemap};
use github_v3::PullRequester;
use github_v3::Authorization;
use github_v3::types::{GitTm, Repository, GitErr};
use github_v3::pull_request_types::{PullRequestQuery, PullRequestStateQuery};
use github_v3::issue_comment_types::{IssueComment, CreateComment};

use std::thread;

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

struct EventInfo;
impl typemap::Key for EventInfo { type Value = EventType; }

struct IsIssueComment;
impl typemap::Key for IsIssueComment { type Value = IssueCommentEvent; }

struct IsPullRequestReviewComment;
impl typemap::Key for IsPullRequestReviewComment { type Value = PullRequestReviewCommentEvent; }

struct IsPullRequest;
impl typemap::Key for IsPullRequest { type Value = PullRequestEvent; }

struct IsPush;
impl typemap::Key for IsPush { type Value = PushEvent; }


impl BeforeMiddleware for Deserialize {
  fn before(&self, req: &mut Request) -> IronResult<()> {
    let mut payload = String::new();
    req.body.read_to_string(&mut payload).unwrap();

    let headers = req.headers.clone();
    let event_header = headers.get::<GithubEventHeader>();
    event_header.map(|header| {
      match header.event_name.as_ref() {
        "issue_comment" => {
          req.extensions.insert::<EventInfo>(EventType::IssueComment);
          req.extensions.insert::<IsIssueComment>(json::decode(&payload).unwrap());
        },
        "pull_request_review_comment" => {
          req.extensions.insert::<EventInfo>(EventType::PullRequestReviewComment);
          req.extensions.insert::<IsPullRequestReviewComment>(json::decode(&payload).unwrap());
        },
        "pull_request" => {
          req.extensions.insert::<EventInfo>(EventType::PullRequest);
          req.extensions.insert::<IsPullRequest>(json::decode(&payload).unwrap());
        },
        "push" => {
          req.extensions.insert::<EventInfo>(EventType::Push);
          req.extensions.insert::<IsPush>(json::decode(&payload).unwrap());
        },
        _ => {req.extensions.insert::<EventInfo>(EventType::UnknownEvent);}
      }
    });
    Ok(())
  }
}


fn handle_webhooks(req: &mut Request) -> IronResult<Response> {
  let possible_event_type = req.extensions.get::<EventInfo>();

  match possible_event_type {
    Some(&EventType::IssueComment) => Ok(Response::with((status::Accepted, "{\"body\":\"Successful recv of issue comment\"}"))),
    Some(&EventType::PullRequestReviewComment) => Ok(Response::with((status::Accepted, "{\"body\":\"Successful recv of pull request review comment\"}"))),
    Some(&EventType::PullRequest) => Ok(Response::with((status::Accepted, "{\"body\":\"Successful recv of pull request\"}"))),
    Some(&EventType::Push) => Ok(Response::with((status::Accepted, "{\"body\":\"Successful recv of push\"}"))),
    Some(&EventType::UnknownEvent) => Ok(Response::with((status::Accepted, "{\"body\":\"Recv an unhandled event\"}"))),
    None => Ok(Response::with((status::Accepted, "{\"body\":\"No event header provided\"}")))
  }
}

struct DeliverActionables {
  issue_comment_tx: Mutex<Sender<IssueCommentEvent>>,
  pull_request_review_tx: Mutex<Sender<PullRequestReviewCommentEvent>>,
  pull_request_tx: Mutex<Sender<PullRequestEvent>>,
}

impl AfterMiddleware for DeliverActionables {
  fn after(&self, req: &mut Request, response: Response) -> IronResult<Response> {
    let possible_event_type = req.extensions.remove::<EventInfo>();
    match possible_event_type {
      Some(EventType::IssueComment) => {
        let possible_payload = req.extensions.remove::<IsIssueComment>();
        possible_payload.map(|payload: IssueCommentEvent| self.issue_comment_tx.lock().map(|sender| sender.send(payload)));

      },
      Some(EventType::PullRequestReviewComment) => {
        let possible_payload = req.extensions.remove::<IsPullRequestReviewComment>();
        possible_payload.map(|payload: PullRequestReviewCommentEvent| self.pull_request_review_tx.lock().map(|sender| sender.send(payload)));
      },
      Some(EventType::PullRequest) => {
        let possible_payload = req.extensions.remove::<IsPullRequest>();
        possible_payload.map(|payload: PullRequestEvent| self.pull_request_tx.lock().map(|sender| sender.send(payload)));
      },
      _ => ()
    }
    Ok(response)
  }
}

fn main() {
  let token = std::env::var("CATALYST_GITHUB_OAUTH_TOKEN").unwrap();
  let repo_owner = std::env::var("CATALYST_REPO_OWNER").unwrap();
  let repo_name = std::env::var("CATALYST_REPO_NAME").unwrap();
  let (issue_comment_tx, issue_comment_rx) = channel();
  let (pull_request_tx, pull_request_rx) = channel();
  let (pull_request_review_tx, pull_request_review_rx) = channel();

  let deliverer = DeliverActionables {
    issue_comment_tx: Mutex::new(issue_comment_tx),
    pull_request_tx: Mutex::new(pull_request_tx),
    pull_request_review_tx: Mutex::new(pull_request_review_tx),
  };
  let mut webhook_chain = Chain::new(handle_webhooks);
  webhook_chain.link_before(Deserialize);
  webhook_chain.link_after(deliverer);

  let mut router = Router::new();
  router.get("/", handle_root);
  router.post("/github_webhooks", webhook_chain);

  thread::spawn (move || {
    let mut channels_up = true;
    let client = GithubClient::new(Some(Authorization("token ".to_owned() + &token)));
    while channels_up {
      let possible_issue_comment = issue_comment_rx.try_recv();
      let possible_pull_request = pull_request_rx.try_recv();
      let possible_pull_request_review = pull_request_review_rx.try_recv();

      let _ = possible_issue_comment
        .map(|issue_comment| {
          if issue_comment.comment.body.clone().contains("pt r?") {
            let issue_id = issue_comment.issue.number.clone();
            let repo = Repository{ owner: repo_owner.clone(), repo_name: repo_name.clone() };
            let response = CreateComment { body: "PTBOT: Assigning @acmcarther to this PR".to_owned() };
            println!("LOG: Received a request for reviewers on issue {}, assigning @acmcarther and @seanstrom", issue_id);
            client.create_comment(repo, issue_id, response);
          }
        })
        .map_err(|err| if err == TryRecvError::Disconnected {channels_up = false});
      let _ = possible_pull_request.map_err(|err| if err == TryRecvError::Disconnected {channels_up = false});
      let _ = possible_pull_request_review.map_err(|err| if err == TryRecvError::Disconnected {channels_up = false});

      //possible_pull_request.map(|err| if err == TryRecvError::Disconnected {channels_up = false});
      //possible_pull_request_review.map(|err| if err == TryRecvError::Disconnected {channels_up = false});

      thread::sleep_ms(2000);
    }
  });

  Iron::new(router).http("0.0.0.0:8080").unwrap();
}
