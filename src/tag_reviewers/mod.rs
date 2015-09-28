use github_v3::IssueCommenter;
use github_v3::types::{
  Message,
  IssueId
};
use github_v3::types::repos::{
  Repository,
  RepoName
};

use github_v3::types::users::{
  UserName
};
use github_v3::types::comments::{
  IssueCommentEvent,
  CreateIssueComment,
};

use rand::{thread_rng, sample};

use itertools::Itertools;

const HARDCODED_REVIEWERS: [&'static str; 2] = ["acmcarther", "seanstrom"];

pub fn tag<T:IssueCommenter>(issue_comment: &IssueCommentEvent, client: &T) {
  let issue_id = issue_comment.issue.number.clone();
  let repo_name = issue_comment.repository.name.clone();
  let user_name = issue_comment.repository.owner.login.clone();
  let comment_body = issue_comment.comment.body.clone();
  easy_tag(issue_id, repo_name, user_name, comment_body, client);
}

pub fn easy_tag<T:IssueCommenter>(issue_id: IssueId, repo_name: RepoName, user_name: UserName, comment_body: Message, client: &T) {
  let contained_comment = Some(comment_body);
  let _ = contained_comment
    .into_iter()
    .filter(|comment| comment.contains("pt r?"))
    .map(|comment| {
      match comment.matches("@").count() {
        0 => (comment, 2),
        1 => (comment, 1),
        _ => (comment, 0)
      }
    })
    .filter(|&(_, rec_count)| rec_count != 0)
    .foreach(|(comment, rec_count)| {
      let repo = Repository{ owner: user_name.clone(), repo_name: repo_name.clone() };
      let mut rng = thread_rng();

      let reviewer_list = HARDCODED_REVIEWERS.clone();
      let filtered_reviewers = reviewer_list.iter().filter(|reviewer| !comment.contains(*reviewer));

      let sample = sample(&mut rng, filtered_reviewers, rec_count);
      let reviewers = sample.into_iter().fold("".to_owned(), |acc, &reviewer| acc + "@" + reviewer + " ");
      let response = CreateIssueComment { body: "PTBOT: Assigning ".to_owned() + &reviewers + "to this PR" };
      println!("LOG: Received a request for reviewers on issue {}, assigning {}", issue_id, reviewers);
      let _ = client.create_comment(repo, issue_id, response);
    });
}

#[cfg(test)]
mod tests {
  use expectest::core::expect;
  use expectest::matchers::be_equal_to;

  pub use super::{
    easy_tag
  };

  use github_v3::IssueCommenter;
  use github_v3::types::repos::Repository;

  use rand::{thread_rng, sample};

  use itertools::Itertools;

  use github_v3::types::{
    GitErr,
    IssueId,
  };

  use github_v3::types::comments::{
    CommentId,
    ListIssueCommentsQuery,
    ListRepoCommentsQuery,
    CreateIssueComment,
    EditComment,
    DeleteCommentStatus,
    IssueComment,
  };

  use std::io::ErrorKind;

  use rusty_mock::StubHelper;

  create_stub! {
    IssueCommenterStub {
      list_in_issue(Repository, IssueId, Option<ListIssueCommentsQuery>) -> Result<Vec<IssueComment>, GitErr>,
      list_in_repo(Repository, Option<ListRepoCommentsQuery>) -> Result<Vec<IssueComment>, GitErr>,
      get_comment(Repository, CommentId) -> Result<IssueComment, GitErr>,
      create_comment(Repository, IssueId, CreateIssueComment) -> Result<IssueComment, GitErr>,
      edit_comment(Repository, CommentId, EditComment) -> Result<IssueComment, GitErr>,
      delete_comment(Repository, CommentId) -> Result<DeleteCommentStatus, GitErr>
    }
  }

  instrument_stub! {
    IssueCommenterStub as IssueCommenter {
      {stub create_comment(&self, repo: Repository, issue_id: IssueId, details: CreateIssueComment) -> Result<IssueComment, GitErr>}
      {nostub list_in_issue(&self, repo: Repository, issue_id: IssueId, query: Option<ListIssueCommentsQuery>) -> Result<Vec<IssueComment>, GitErr>}
      {nostub list_in_repo(&self, repo: Repository, query: Option<ListRepoCommentsQuery>) -> Result<Vec<IssueComment>, GitErr>}
      {nostub get_comment(&self, repo: Repository, comment_id: CommentId) -> Result<IssueComment, GitErr>}
      {nostub edit_comment(&self, repo: Repository, comment_id: CommentId, details: EditComment) -> Result<IssueComment, GitErr>}
      {nostub delete_comment(&self, repo: Repository, comment_id: CommentId) -> Result<DeleteCommentStatus, GitErr>}
    }
  }

  #[test]
  fn it_does_nothing_when_the_review_caller_string_is_not_present() {
    let mut stub = IssueCommenterStub::new();
    stub.create_comment.returns(Err(GitErr::NotImplemented("Testing".to_owned())));
    easy_tag(1, "repo_name".to_owned(), "user_name".to_owned(), "comment".to_owned(), &stub);
    expect!(stub.create_comment.was_called()).to(be_equal_to(false));
  }

  #[test]
  fn it_does_nothing_when_two_people_already_tagged() {
    let mut stub = IssueCommenterStub::new();
    stub.create_comment.returns(Err(GitErr::NotImplemented("Testing".to_owned())));
    easy_tag(1, "repo_name".to_owned(), "user_name".to_owned(), "pt r? @1 @2".to_owned(), &stub);
    expect!(stub.create_comment.was_called()).to(be_equal_to(false));
  }

  #[test]
  fn it_tags_someone_if_one_person_was_tagged() {
    let mut stub = IssueCommenterStub::new();
    stub.create_comment.returns(Err(GitErr::NotImplemented("Testing".to_owned())));
    easy_tag(1, "repo_name".to_owned(), "user_name".to_owned(), "pt r? @1".to_owned(), &stub);
    expect!(stub.create_comment.was_called_once()).to(be_equal_to(true));
    let (_, _, call_comment) = stub.create_comment.get_args_for_call(0).unwrap();
    // Since it's random who gets called, we have to check for both
    expect!(call_comment.body.contains("@acmcarther") || call_comment.body.contains("@seanstrom")).to(be_equal_to(true));
  }

  #[test]
  fn it_tags_two_people_if_nobody_was_tagged() {
    let mut stub = IssueCommenterStub::new();
    stub.create_comment.returns(Err(GitErr::NotImplemented("Testing".to_owned())));
    easy_tag(1, "repo_name".to_owned(), "user_name".to_owned(), "pt r?".to_owned(), &stub);

    expect!(stub.create_comment.was_called()).to(be_equal_to(true));
    let (_, _, call_comment) = stub.create_comment.get_args_for_call(0).unwrap();

    // We assert that both people got tagged
    expect!(call_comment.body.contains("@acmcarther") && call_comment.body.contains("@seanstrom")).to(be_equal_to(true));
  }

  #[test]
  fn it_does_not_retag_people_already_tagged() {
    let mut stub = IssueCommenterStub::new();
    stub.create_comment.returns(Err(GitErr::NotImplemented("Testing".to_owned())));


    // Since tagging is randomized, we should get a "significant" sample to verify this test
    for call_idx in 0..10 {
      easy_tag(1, "repo_name".to_owned(), "user_name".to_owned(), "@acmcarther pt r?".to_owned(), &stub);
      expect!(stub.create_comment.was_called_n_times(call_idx + 1)).to(be_equal_to(true));
      let (_, _, call_comment) = stub.create_comment.get_args_for_call(call_idx as usize).unwrap();
      expect!(call_comment.body.clone()).to(be_equal_to("PTBOT: Assigning @seanstrom to this PR"));
    }
  }
}
