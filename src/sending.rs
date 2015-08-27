pub use self::sending::{
  spawn_sender
};

mod sending {

  use github_v3::{GithubClient, IssueCommenter};

  use github_v3::event_types::{
    IssueCommentEvent,
    PullRequestEvent,
    PullRequestReviewCommentEvent,
  };

  use std::sync::mpsc::{Receiver, TryRecvError};

  use github_v3::Authorization;
  use github_v3::types::Repository;
  use github_v3::issue_comment_types::CreateComment;

  use std::thread;

  use std::env;

  use std::thread::JoinHandle;

  pub fn spawn_sender(
    issue_comment_rx: Receiver<IssueCommentEvent>,
    pull_request_rx: Receiver<PullRequestEvent>,
    pull_request_review_rx: Receiver<PullRequestReviewCommentEvent>
    ) -> JoinHandle<()> {

    let token = env::var("CATALYST_GITHUB_OAUTH_TOKEN").unwrap();
    let repo_owner = env::var("CATALYST_REPO_OWNER").unwrap();
    let repo_name = env::var("CATALYST_REPO_NAME").unwrap();

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
              println!("LOG: Received a request for reviewers on issue {}, assigning @acmcarther", issue_id);
              let _ = client.create_comment(repo, issue_id, response);
            }
          })
          .map_err(|err| if err == TryRecvError::Disconnected {channels_up = false});
        let _ = possible_pull_request.map_err(|err| if err == TryRecvError::Disconnected {channels_up = false});
        let _ = possible_pull_request_review.map_err(|err| if err == TryRecvError::Disconnected {channels_up = false});

        //possible_pull_request.map(|err| if err == TryRecvError::Disconnected {channels_up = false});
        //possible_pull_request_review.map(|err| if err == TryRecvError::Disconnected {channels_up = false});

        thread::sleep_ms(2000);
      }
    })
  }

}
