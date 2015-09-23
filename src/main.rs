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

mod listening;
mod sending;
mod client_api;
mod webhooks;
mod tag_reviewers;

use std::sync::mpsc::channel;

fn main() {
  let (issue_comment_tx, issue_comment_rx) = channel();
  let (pull_request_tx, pull_request_rx) = channel();
  let (pull_request_review_tx, pull_request_review_rx) = channel();

  let sender = sending::spawn_sender(issue_comment_rx, pull_request_rx, pull_request_review_rx);

  listening::spawn_listener(issue_comment_tx, pull_request_tx, pull_request_review_tx).http("0.0.0.0:8080").unwrap();

}
