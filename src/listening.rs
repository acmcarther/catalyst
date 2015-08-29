pub use self::listening::{
  spawn_listener
};

mod listening {
  use iron::prelude::*;
  use iron::status;
  use router::Router;
  use iron::headers::{parsing, Header, HeaderFormat};
  use iron::error::HttpError;

  use std::io::Read;

  use std::fmt;
  use std::ascii::AsciiExt;
  use std::sync::Mutex;

  use github_v3::types::comments::{
    IssueCommentEvent,
    PullRequestReviewCommentEvent,
  };
  use github_v3::types::pull_requests::PullRequestEvent;
  use github_v3::types::PushEvent;

  use rustc_serialize::{json};
  use std::sync::mpsc::Sender;

  use iron::{BeforeMiddleware, AfterMiddleware, typemap};


  fn handle_root(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Hello from Catalyst. At some point a configuration page will live here.")))
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

  pub fn spawn_listener(
    issue_comment_tx: Sender<IssueCommentEvent>,
    pull_request_tx: Sender<PullRequestEvent>,
    pull_request_review_tx: Sender<PullRequestReviewCommentEvent>
    ) -> Iron<Router> {

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

    Iron::new(router)
  }
}
