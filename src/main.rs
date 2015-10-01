extern crate github_v3;
extern crate iron;
extern crate time;
extern crate router;
extern crate rustc_serialize;
extern crate itertools;
extern crate staticfile;
extern crate mount;
extern crate rand;

#[cfg(test)]
#[macro_use(expect)]
extern crate expectest;

#[cfg(test)]
#[macro_use(create_stub, instrument_stub, impl_helper)]
extern crate rusty_mock;

mod listening;
mod client_api;
mod webhooks;
mod commenter;

use commenter::Commenter;

use std::sync::mpsc::channel;

fn main() {
  let (issue_comment_tx, issue_comment_rx) = channel();
  let (pull_request_tx, pull_request_rx) = channel();
  let (pull_request_review_tx, pull_request_review_rx) = channel();

  let commenter = Commenter::new(issue_comment_rx, pull_request_rx, pull_request_review_rx);

  let commenter_join_guard = commenter.start();

  listening::spawn_listener(issue_comment_tx, pull_request_tx, pull_request_review_tx).http("0.0.0.0:8080").unwrap();

}
