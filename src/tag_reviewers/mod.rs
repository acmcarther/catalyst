use github_v3::IssueCommenter;
use github_v3::Authorization;
use github_v3::github_client::GithubClient;
use github_v3::types::repos::Repository;
use github_v3::types::pull_requests::{
  PullRequestEvent,
};
use github_v3::types::comments::{
  IssueCommentEvent,
  CreateIssueComment,
  PullRequestReviewCommentEvent,
};

use std::env;
use std::thread;
use std::thread::JoinHandle;
use std::collections::HashMap;
use std::sync::mpsc::{Receiver, TryRecvError};

use itertools::Itertools;

use rand::{thread_rng, sample};

const HARDCODED_REVIEWERS: [&'static str; 2] = ["acmcarther", "seanstrom"];

// TODO: Mad tests. This should be totally possible since this trait is so simple
pub fn tag<T:IssueCommenter>(issue_comment: &IssueCommentEvent, client: &T) {
  let contained_comment = Some(issue_comment.comment.body.clone());
  contained_comment
    .into_iter()
    .filter(|comment| comment.contains("pt r?"))
    .map(|comment| {
      match comment.matches("@").count() {
        0 => 2,
        1 => 1,
        _ => 0
      }
    })
    .filter(|recCount| *recCount == 0)
    .map(|recCount| {
      let issue_id = issue_comment.issue.number.clone();
      let name = issue_comment.repository.name.clone();
      let owner = issue_comment.repository.owner.login.clone();
      let repo = Repository{ owner: owner, repo_name: name };

      let mut rng = thread_rng();
      let possible_reviewers = HARDCODED_REVIEWERS.clone();
      let sample = sample(&mut rng, possible_reviewers.iter(), 1);
      let reviewers = sample.into_iter().fold("".to_owned(), |acc, &reviewer| acc + "@" + reviewer + " ");

      let response = CreateIssueComment { body: "PTBOT: Assigning ".to_owned() + &reviewers + "to this PR" };
      println!("LOG: Received a request for reviewers on issue {}, assigning {}", issue_id, reviewers);
      let _ = client.create_comment(repo, issue_id, response);

    });
}
