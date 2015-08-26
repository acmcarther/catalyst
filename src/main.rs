extern crate github_v3;
extern crate iron;
extern crate time;
extern crate router;

mod listening;
mod sending;
mod state;

use iron::prelude::*;
use iron::status;
use router::Router;
use std::env::args;
use std::path::PathBuf;

fn handle_root(req: &mut Request) -> IronResult<Response> {
  Ok(Response::with((status::Ok, "cheapassbox.com rust + iron & soon catalyst")))
}

fn main() {
  let token = std::env::var("CATALYST_GITHUB_OAUTH_TOKEN").unwrap();
  let repo_owner = std::env::var("CATALYST_REPO_OWNER").unwrap();
  let repo_name = std::env::var("CATALYST_REPO_NAME").unwrap();

  let mut router = Router::new();

  router.get("/", handle_root);

  //listening::start_listener(token, repo_owner, repo_name)
  Iron::new(router).http("localhost:8080").unwrap();
}
