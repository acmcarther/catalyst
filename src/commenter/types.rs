use github_v3::types::pull_requests::{
  PullRequestEvent,
};
use github_v3::types::comments::{
  IssueCommentEvent,
  PullRequestReviewCommentEvent,
};

pub enum HandledGithubEvents {
  IssueCommentEvent(IssueCommentEvent),
  PullRequestReviewCommentEvent(PullRequestReviewCommentEvent),
  PullRequestEvent(PullRequestEvent)
}
