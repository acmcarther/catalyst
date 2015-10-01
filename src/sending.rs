use github_v3::Authorization;
use github_v3::github_client::GithubClient;
use github_v3::types::pull_requests::{
  PullRequestEvent,
};
use github_v3::types::comments::{
  IssueCommentEvent,
  PullRequestReviewCommentEvent,
};

use std::env;
use std::thread;
use std::thread::JoinHandle;
use std::collections::HashMap;
use std::sync::mpsc::{Receiver, TryRecvError};

use itertools::Itertools;

use tag_reviewers;

type RepoFullName = String;
type RepoStatistics = ();

fn contains_monitored_repo(event: &IssueCommentEvent, monitored_repos: &HashMap<RepoFullName, RepoStatistics>) -> bool {
  monitored_repos.contains_key(&event.repository.full_name)
}

pub fn spawn_sender(
  issue_comment_rx: Receiver<IssueCommentEvent>,
  pull_request_rx: Receiver<PullRequestEvent>,
  pull_request_review_rx: Receiver<PullRequestReviewCommentEvent>
  ) -> JoinHandle<()> {

  let token = env::var("CATALYST_GITHUB_OAUTH_TOKEN").unwrap();
  let repo_owner = env::var("CATALYST_REPO_OWNER").unwrap();
  let repo_name = env::var("CATALYST_REPO_NAME").unwrap();

  let mut monitored_repos = HashMap::new();
  monitored_repos.insert(repo_owner.clone() + "/" + &repo_name, ());

  let client = GithubClient::new(Some(Authorization("token ".to_owned() + &token)));

  thread::spawn (move || {
    let mut channels_up = true;
    while channels_up {
      let possible_issue_comment = issue_comment_rx.try_recv();
      let possible_pull_request = pull_request_rx.try_recv();
      let possible_pull_request_review = pull_request_review_rx.try_recv();

      let _ = possible_issue_comment
        .map_err(|err| if err == TryRecvError::Disconnected {channels_up = false})
        .ok()
        .iter()
        .filter(|issue_comment| contains_monitored_repo(issue_comment, &monitored_repos))
        .foreach(|issue_comment| tag_reviewers::tag(&issue_comment, &client));
      let _ = possible_pull_request.map_err(|err| if err == TryRecvError::Disconnected {channels_up = false});
      let _ = possible_pull_request_review.map_err(|err| if err == TryRecvError::Disconnected {channels_up = false});

      //possible_pull_request.map(|err| if err == TryRecvError::Disconnected {channels_up = false});
      //possible_pull_request_review.map(|err| if err == TryRecvError::Disconnected {channels_up = false});

      thread::sleep_ms(2000);
    }
  })
}
