pub use self::listening::{
  start_listener
};

mod listening {
  use std::thread;
  use github_v3::GithubClient;
  use github_v3::IssueCommenter;
  use github_v3::PullRequester;
  use github_v3::Authorization;
  use github_v3::types::{GitTm, Repository, GitErr};
  use github_v3::pull_request_types::{PullRequestQuery, PullRequestStateQuery};
  use github_v3::issue_comment_types::{IssueComment, CreateComment};

  pub fn start_listener(token: String, repo_owner: String, repo_name: String) {

    let client = GithubClient::new(Some(Authorization("token ".to_owned() + &token)));
    loop {
      let repo = Repository{ owner: repo_owner.clone(), repo_name: repo_name.clone() };

      let query = PullRequestQuery { state: Some(PullRequestStateQuery::Closed), head: None, base: None, sort: None, direction: None};
      let all_prs = client.list(repo, Some(query));
      let comment_results = all_prs.map (|all_prs| {
        println!("all_prs {:?}", all_prs);
        all_prs.into_iter().map(|pr| {
          let issue_id = pr.number;
          let repo = Repository{ owner: repo_owner.clone(), repo_name: repo_name.clone() };
          let all_comments = client.list_in_issue(repo, issue_id.clone(), None);
          all_comments.map(|comments| {
            println!("comments {:?}", comments);
            comments
              .into_iter()
              .filter(|comment| comment.body.contains("pt r?"))
              .map(|comment| {
                let repo = Repository{ owner: repo_owner.clone(), repo_name: repo_name.clone() };
                let issue_id = issue_id.clone();
                client.create_comment(repo, issue_id, CreateComment { body: "PTBOT: Assigning @acmcarther to this PR".to_owned()})
              }).collect::<Vec<Result<IssueComment, GitErr>>>()
          })
        }).collect::<Vec<Result<Vec<Result<IssueComment, GitErr>>, GitErr>>>()
      });
      println!("result: {:?}", comment_results);
      thread::sleep_ms(100000);
    }
  }
}
