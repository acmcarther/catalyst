use github_v3::IssueCommenter;
use github_v3::types::repos::Repository;
use github_v3::types::comments::CreateIssueComment;

use types::SimplifiedIssueCommentEvent;

use rand::{thread_rng, sample};

use itertools::Itertools;

pub struct ReviewTagger {
  reviewers: Vec<String>
}

impl ReviewTagger {
  pub fn new(reviewers: Vec<String>) -> ReviewTagger {
    ReviewTagger { reviewers: reviewers }
  }
}

pub trait Tagger {
  fn tag<T:IssueCommenter>(&self, event: &SimplifiedIssueCommentEvent, client: &T);
}

impl Tagger for ReviewTagger {
  fn tag<T:IssueCommenter>(&self, event: &SimplifiedIssueCommentEvent, client: &T) {
    let issue_id = event.issue_number.clone();
    let repo_name = event.full_repo_name.clone();
    let owner_name = event.owner_name.clone();
    let sender_name = event.sender_name.clone();
    let contained_comment = Some(event.comment_body.clone());
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
      .map(|(comment, rec_count)| {
        let filtered_reviewers = self.reviewers.iter()
          .filter(|reviewer| **reviewer != sender_name)
          .filter(|reviewer| !comment.contains(*reviewer))
          // TODO: No clone & collect here, must solve the is_empty() problem for iterator
          .map(|reviewer| reviewer.clone())
          .collect::<Vec<String>>();
        (rec_count, filtered_reviewers)
      })
      .filter(|&(_, ref available_reviewers)| !available_reviewers.is_empty())
      .foreach(|(rec_count, available_reviewers)| {
        let repo = Repository{ owner: owner_name.clone(), repo_name: repo_name.clone() };
        let mut rng = thread_rng();
        let sample = sample(&mut rng, available_reviewers.iter(), rec_count);
        let reviewers = sample.into_iter().fold("".to_owned(), |acc, ref reviewer| acc + "@" + &reviewer + " ");
        let response = CreateIssueComment { body: "PTBOT: Assigning ".to_owned() + &reviewers + "to this PR" };
        println!("LOG: Received a request for reviewers on issue {}, assigning {}", issue_id, reviewers);
        let _ = client.create_comment(repo, issue_id, response);
      });
  }
}

#[cfg(test)]
mod tests {
  use rusty_mock::*;

  use expectest::core::expect;
  use expectest::matchers::be_equal_to;

  use super::*;

  use github_v3::IssueCommenter;
  use github_v3::types::repos::Repository;

  use types::SimplifiedIssueCommentEvent;
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

  struct IssueCommenterStub {
    create_comment: ArgWatchingStub<Result<IssueComment, GitErr>, (Repository, IssueId, CreateIssueComment)>
  }

  impl IssueCommenterStub {
    fn new() -> IssueCommenterStub {
      IssueCommenterStub {
        create_comment: ArgWatchingStub::new()
      }
    }
  }

  instrument_stub! {
    IssueCommenterStub as IssueCommenter {
      {ArgWatchingStub: create_comment(&self, repo: Repository, issue_id: IssueId, details: CreateIssueComment) -> Result<IssueComment, GitErr>}
      {nostub: list_in_issue(&self, repo: Repository, issue_id: IssueId, query: Option<ListIssueCommentsQuery>) -> Result<Vec<IssueComment>, GitErr>}
      {nostub: list_in_repo(&self, repo: Repository, query: Option<ListRepoCommentsQuery>) -> Result<Vec<IssueComment>, GitErr>}
      {nostub: get_comment(&self, repo: Repository, comment_id: CommentId) -> Result<IssueComment, GitErr>}
      {nostub: edit_comment(&self, repo: Repository, comment_id: CommentId, details: EditComment) -> Result<IssueComment, GitErr>}
      {nostub: delete_comment(&self, repo: Repository, comment_id: CommentId) -> Result<DeleteCommentStatus, GitErr>}
    }
  }

  fn default_stub() -> IssueCommenterStub {
    let mut stub = IssueCommenterStub::new();
    stub.create_comment.returns(Err(GitErr::NotImplemented("Testing".to_owned())));
    stub
  }

  fn issue_comment_event_with_comment(comment: &str) -> SimplifiedIssueCommentEvent {
    issue_comment_event_with_comment_and_commenter(comment, "sender_name")
  }

  fn issue_comment_event_with_comment_and_commenter(comment: &str, commenter: &str) -> SimplifiedIssueCommentEvent {
    SimplifiedIssueCommentEvent::new("repo_name".to_owned(), 1, "user_name".to_owned(), commenter.to_owned(), comment.to_owned())
  }

  #[test]
  fn it_does_nothing_when_the_review_caller_string_is_not_present() {
    let stub = default_stub();

    let event = issue_comment_event_with_comment("comment");
    ReviewTagger::new(vec!["acmcarther".to_owned(), "seanstrom".to_owned()]).tag(&event, &stub);
    expect!(stub.create_comment.was_called()).to(be_equal_to(false));
  }

  #[test]
  fn it_calls_create_comment_with_the_issue() {
    let stub = default_stub();

    let event = issue_comment_event_with_comment("pt r?");
    ReviewTagger::new(vec!["acmcarther".to_owned(), "seanstrom".to_owned()]).tag(&event, &stub);
    expect!(stub.create_comment.was_called_once()).to(be_equal_to(true));
    let (_, issue_id, _) = stub.create_comment.get_args_for_call(0).unwrap();
    expect!(issue_id).to(be_equal_to(1));
  }

  #[test]
  fn it_calls_create_comment_with_the_repo() {
    let stub = default_stub();

    let event = issue_comment_event_with_comment("pt r?");
    ReviewTagger::new(vec!["acmcarther".to_owned(), "seanstrom".to_owned()]).tag(&event, &stub);
    expect!(stub.create_comment.was_called_once()).to(be_equal_to(true));
    let (repo, _, _) = stub.create_comment.get_args_for_call(0).unwrap();
    let expected_repo = Repository { owner: "user_name".to_owned(), repo_name: "repo_name".to_owned() };
    expect!(repo).to(be_equal_to(expected_repo));
  }

  #[test]
  fn it_does_nothing_when_two_people_already_tagged() {
    let stub = default_stub();
    let event = issue_comment_event_with_comment("@acmcarther, @someone_else pt r?");
    ReviewTagger::new(vec!["acmcarther".to_owned(), "seanstrom".to_owned()]).tag(&event, &stub);
    expect!(stub.create_comment.was_called()).to(be_equal_to(false));
  }

  #[test]
  fn it_tags_someone_if_one_person_was_tagged() {
    let stub = default_stub();

    let event = issue_comment_event_with_comment("pt r? @1");
    ReviewTagger::new(vec!["acmcarther".to_owned(), "seanstrom".to_owned()]).tag(&event, &stub);
    expect!(stub.create_comment.was_called_once()).to(be_equal_to(true));
    let (_, _, call_comment) = stub.create_comment.get_args_for_call(0).unwrap();
    // Since it's random who gets called, we have to check for both
    expect!(call_comment.body.contains("@acmcarther") || call_comment.body.contains("@seanstrom")).to(be_equal_to(true));
  }

  #[test]
  fn it_tags_two_people_if_nobody_was_tagged() {
    let stub = default_stub();

    let event = issue_comment_event_with_comment("pt r?");
    ReviewTagger::new(vec!["acmcarther".to_owned(), "seanstrom".to_owned()]).tag(&event, &stub);
    expect!(stub.create_comment.was_called()).to(be_equal_to(true));
    let (_, _, call_comment) = stub.create_comment.get_args_for_call(0).unwrap();
    expect!(call_comment.body.contains("@acmcarther") && call_comment.body.contains("@seanstrom")).to(be_equal_to(true));
  }

  #[test]
  fn it_does_not_retag_people_already_tagged() {
    let stub = default_stub();
    let tagger = ReviewTagger::new(vec!["acmcarther".to_owned(), "seanstrom".to_owned()]);

    // Since tagging is randomized, we should get a "significant" sample to verify this test
    for call_idx in 0..10 {
      let event = issue_comment_event_with_comment("@acmcarther pt r?");
      tagger.tag(&event, &stub);
      expect!(stub.create_comment.was_called_n_times(call_idx + 1)).to(be_equal_to(true));
      let (_, _, call_comment) = stub.create_comment.get_args_for_call(call_idx as usize).unwrap();
      expect!(call_comment.body.clone()).to(be_equal_to("PTBOT: Assigning @seanstrom to this PR"));
    }
  }

  #[test]
  fn it_does_not_tag_the_commenter() {
    let stub = default_stub();
    let tagger = ReviewTagger::new(vec!["acmcarther".to_owned(), "seanstrom".to_owned()]);

    // Since tagging is randomized, we should get a "significant" sample to verify this test
    for call_idx in 0..10 {
      let event = issue_comment_event_with_comment_and_commenter("@someone_else pt r?", "seanstrom");
      tagger.tag(&event, &stub);
      expect!(stub.create_comment.was_called_n_times(call_idx + 1)).to(be_equal_to(true));
      let (_, _, call_comment) = stub.create_comment.get_args_for_call(call_idx as usize).unwrap();
      expect!(call_comment.body.clone()).to(be_equal_to("PTBOT: Assigning @acmcarther to this PR"));
    }
  }

  #[test]
  fn it_does_not_tag_anyone_if_theres_nobody_left_to_tag() {
    let stub = default_stub();
    let event = issue_comment_event_with_comment_and_commenter("@acmcarther pt r?", "seanstrom");

    ReviewTagger::new(vec!["acmcarther".to_owned(), "seanstrom".to_owned()]).tag(&event, &stub);
    expect!(stub.create_comment.was_called()).to(be_equal_to(false));
  }
}
