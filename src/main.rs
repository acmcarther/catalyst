extern crate iron;
extern crate time;
extern crate router;
extern crate rustc_serialize;
extern crate itertools;
extern crate staticfile;
extern crate mount;
extern crate rand;

extern crate coffeelint_parser;

#[macro_use]
extern crate github_v3;

#[cfg(test)]
#[macro_use(expect)]
extern crate expectest;

#[cfg(test)]
#[macro_use]
extern crate rusty_mock;

mod types;
mod listening;
mod client_api;
mod webhooks;
mod commenter;
mod continuous_integrator;

use commenter::Commenter;

use std::env;
use std::sync::mpsc::channel;

fn main() {
  let (event_tx, event_rx) = channel();
  let (build_tx, build_rx) = channel();

  let auth_token = env::var("CATALYST_GITHUB_OAUTH_TOKEN").unwrap_or("dummy_token".to_owned());
  let mut commenter = Commenter::new(event_rx, build_rx, auth_token);
  commenter.add_repo("acmcarther/catalyst", vec![
    "acmcarther".to_owned(),
    "seanstrom".to_owned(),
    "rschifflin".to_owned()
  ]);

  let commenter_join_guard = commenter.start();

  listening::spawn_listener(event_tx, build_tx)
    .http("0.0.0.0:8080")
    .unwrap();

  commenter_join_guard.join().unwrap();
}
