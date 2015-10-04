mod model_builder;
mod review_tagger;
mod lint_watcher;

use github_v3::Authorization;
use github_v3::github_client::GithubClient;
use github_v3::types::pull_requests::PullRequestEventType;

use std::thread;
use std::thread::JoinHandle;
use std::collections::HashMap;
use std::sync::mpsc::{Receiver, TryRecvError};

use types::{
  SimplifiedIssueCommentEvent,
  HandledGithubEvents
};

use self::review_tagger::*;

type RepoFullName = String;

pub struct Commenter {
  event_rx: Receiver<HandledGithubEvents>,
  monitored_repos: HashMap<RepoFullName, review_tagger::ReviewTagger>,
  client: GithubClient<String>
}

impl Commenter {
  pub fn new(event_rx: Receiver<HandledGithubEvents>, auth_token: String) -> Commenter {
    let monitored_repos = HashMap::new();
    let client = GithubClient::new(Some(Authorization("token ".to_owned() + &auth_token)));

    Commenter {
      event_rx: event_rx,
      monitored_repos: monitored_repos,
      client: client,
    }
  }

  pub fn add_repo(&mut self, repo_full_name: &str, reviewers: Vec<String>) {
    self.monitored_repos.insert(repo_full_name.to_owned(), ReviewTagger::new(reviewers));
  }

  pub fn start(mut self) -> JoinHandle<()> {
    thread::spawn (move || {
      let mut channels_up = true;
      while channels_up {
        channels_up = self.check_event_stream();
        thread::sleep_ms(2000);
      }
    })
  }

  pub fn check_event_stream(&mut self) -> bool {
    let possible_event = self.event_rx.try_recv();

    match possible_event {
      Err(err) => !(err == TryRecvError::Disconnected),
      Ok(event) => {
        match event {
          HandledGithubEvents::IssueCommentEvent(ref e) => {
            self.monitored_repos
              .get(&e.repository.full_name)
              .map(|tagger| tagger.tag(&SimplifiedIssueCommentEvent::from_issue_comment_event(e), &self.client));
          },
          HandledGithubEvents::PullRequestEvent(ref e) => {
            if e.action == PullRequestEventType::Opened {
              self.monitored_repos
                .get(&e.repository.full_name)
                .map(|tagger| tagger.tag(&SimplifiedIssueCommentEvent::from_pull_request_event(e), &self.client));
            }
          },
          _ => ()
        }
        true
      }
    }
  }
}
