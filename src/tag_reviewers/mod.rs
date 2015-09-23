use github_v3::IssueCommenter;
use github_v3::types::repos::Repository;
use github_v3::types::comments::{
  IssueCommentEvent,
  CreateIssueComment,
};

use rand::{thread_rng, sample};

use itertools::Itertools;

const HARDCODED_REVIEWERS: [&'static str; 2] = ["acmcarther", "seanstrom"];

// TODO: Mad tests. This should be totally possible since this trait is so simple
pub fn tag<T:IssueCommenter>(issue_comment: &IssueCommentEvent, client: &T) {
  let contained_comment = Some(issue_comment.comment.body.clone());
  let _ = contained_comment
    .into_iter()
    .filter(|comment| comment.contains("pt r?"))
    .map(|comment| {
      match comment.matches("@").count() {
        0 => 2,
        1 => 1,
        _ => 0
      }
    })
    .filter(|rec_count| *rec_count == 0)
    .foreach(|rec_count| {
      let issue_id = issue_comment.issue.number.clone();
      let name = issue_comment.repository.name.clone();
      let owner = issue_comment.repository.owner.login.clone();
      let repo = Repository{ owner: owner, repo_name: name };

      let mut rng = thread_rng();
      let possible_reviewers = HARDCODED_REVIEWERS.clone();
      let sample = sample(&mut rng, possible_reviewers.iter(), rec_count);
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
    tag
  };

  #[test]
  fn it_does_nothing_when_not_tagged() {
  }

  #[test]
  fn it_does_nothing_when_tagged_but_two_reviewers_are_tagged() {
  }

  #[test]
  fn it_tags_one_person_when_tagged_and_one_reviewer_tagged() {
  }

  #[test]
  fn it_tags_two_people_when_tagged_and_no_reviewers_tagged() {
  }
}
