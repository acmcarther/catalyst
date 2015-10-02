mod model_builder;
mod review_tagger;
mod lint_watcher;
pub mod types;

use github_v3::Authorization;
use github_v3::github_client::GithubClient;
use github_v3::types::comments::IssueCommentEvent;

use std::env;
use std::thread;
use std::thread::JoinHandle;
use std::collections::HashMap;
use std::sync::mpsc::{Receiver, TryRecvError};

type RepoFullName = String;
type RepoStatistics = ();

// Helper
fn contains_monitored_repo(event: &IssueCommentEvent, monitored_repos: &HashMap<RepoFullName, RepoStatistics>) -> bool {
  monitored_repos.contains_key(&event.repository.full_name)
}

pub struct Commenter {
  event_rx: Receiver<types::HandledGithubEvents>,
  monitored_repos: HashMap<RepoFullName, RepoStatistics>,
  client: GithubClient<String>
}


impl Commenter {
  pub fn new(event_rx: Receiver<types::HandledGithubEvents>,) -> Commenter {
    // TODO:  Parameterize off of these, don't read them from environment
    let token = env::var("CATALYST_GITHUB_OAUTH_TOKEN").unwrap_or("dummy_token".to_owned());
    let repo_owner = env::var("CATALYST_REPO_OWNER").unwrap_or("dummy_owner".to_owned());
    let repo_name = env::var("CATALYST_REPO_NAME").unwrap_or("dummy_repo".to_owned());

    let mut monitored_repos = HashMap::new();
    monitored_repos.insert(repo_owner.clone() + "/" + &repo_name, ());

    let client = GithubClient::new(Some(Authorization("token ".to_owned() + &token)));

    Commenter {
      event_rx: event_rx,
      monitored_repos: monitored_repos,
      client: client,
    }
  }

  pub fn start(mut self) -> JoinHandle<()> {
    thread::spawn (move || {
      let mut channels_up = true;
      while channels_up {
        channels_up = self.check_event_stream(review_tagger::tag);
        thread::sleep_ms(2000);
      }
    })
  }


  pub fn check_event_stream<T: Fn(&IssueCommentEvent, &GithubClient<String>)>(&mut self, tagger: T) -> bool {
    let possible_event = self.event_rx.try_recv();

    match possible_event {
      Err(err) => !(err == TryRecvError::Disconnected),
      Ok(event) => {
        match event {
          types::HandledGithubEvents::IssueCommentEvent(ref e) => {
            if contains_monitored_repo(e, &self.monitored_repos) {
              tagger(e, &self.client);
            }
          },
          _ => ()
        }
        true
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use rusty_mock::*;
  use github_v3::github_client::GithubClient;
  use github_v3::types::comments::IssueCommentEvent;

  type TaggerStub = InterceptingStub<(), Fn(&IssueCommentEvent, &GithubClient<String>)>;

  // TODO: Figure out how to test this without building a full IssueCommentEvent, since those are
  //   huge
}
