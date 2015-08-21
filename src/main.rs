extern crate hyper;

mod listening;
mod sending;
mod state;

fn main() {
  let token = std::env::var("CATALYST_GITHUB_OAUTH_TOKEN").unwrap();
  let repo_owner = std::env::var("CATALYST_REPO_OWNER").unwrap();
  let repo_name = std::env::var("CATALYST_REPO_NAME").unwrap();

  listening::start_listener(token, repo_owner, repo_name)
}
