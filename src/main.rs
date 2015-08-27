extern crate github_v3;
extern crate iron;
extern crate time;
extern crate router;
extern crate rustc_serialize;

mod listening;
mod sending;

use std::sync::mpsc::channel;

fn main() {
  let (issue_comment_tx, issue_comment_rx) = channel();
  let (pull_request_tx, pull_request_rx) = channel();
  let (pull_request_review_tx, pull_request_review_rx) = channel();

  let sender = sending::spawn_sender(issue_comment_rx, pull_request_rx, pull_request_review_rx);

  listening::spawn_listener(issue_comment_tx, pull_request_tx, pull_request_review_tx).http("0.0.0.0:8080").unwrap();

}
