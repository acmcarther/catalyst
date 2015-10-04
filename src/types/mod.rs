use github_v3::types::pull_requests::{
  PullRequestEvent,
};
use github_v3::types::comments::{
  IssueCommentEvent,
  PullRequestReviewCommentEvent,
};

pub struct SimplifiedIssueCommentEvent {
  pub full_repo_name: String,
  pub issue_number: u32,
  pub owner_name: String,
  pub comment_body: String,
  pub sender_name: String,
}

impl SimplifiedIssueCommentEvent {
  pub fn new(full_repo_name: String, issue_number: u32, owner_name: String, sender_name: String, comment_body: String) -> SimplifiedIssueCommentEvent {
    SimplifiedIssueCommentEvent {
      full_repo_name: full_repo_name,
      issue_number: issue_number,
      owner_name: owner_name,
      sender_name: sender_name,
      comment_body: comment_body,
    }
  }

  pub fn from_issue_comment_event(e: &IssueCommentEvent) -> SimplifiedIssueCommentEvent {
    SimplifiedIssueCommentEvent {
      full_repo_name: e.repository.name.clone(),
      issue_number: e.issue.number.clone(),
      owner_name: e.repository.owner.login.clone(),
      sender_name: e.sender.login.clone(),
      comment_body: e.comment.body.clone(),
    }
  }
  pub fn from_pull_request_event(e: &PullRequestEvent) -> SimplifiedIssueCommentEvent {
    SimplifiedIssueCommentEvent {
      full_repo_name: e.repository.name.clone(),
      issue_number: e.pull_request.number.clone(),
      owner_name: e.repository.owner.login.clone(),
      sender_name: e.sender.login.clone(),
      comment_body: e.pull_request.body.clone(),
    }
  }
}

pub enum HandledGithubEvents {
  IssueCommentEvent(IssueCommentEvent),
  PullRequestReviewCommentEvent(PullRequestReviewCommentEvent),
  PullRequestEvent(PullRequestEvent)
}
