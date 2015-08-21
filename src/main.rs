extern crate hyper;

use std::io::Read;

use hyper::Client;
use hyper::header::{Authorization, Connection, UserAgent};

fn main() {
  let mut client = Client::new();

  let token = std::env::var("CATALYST_GITHUB_OAUTH_TOKEN").unwrap();
  let repo_owner = std::env::var("CATALYST_REPO_OWNER").unwrap();
  let repo_name = std::env::var("CATALYST_REPO_NAME").unwrap();

  let token_header = Authorization("token ".to_owned() + &token);

  let pulls_url = "https://api.github.com/repos/".to_owned() + &repo_owner + "/" + &repo_name + "/pulls";

  // Creating an outgoing request.
  let mut res = client.get(&pulls_url)
      // set a header
      .header(token_header.clone())
      // set a header
      .header(UserAgent("CatalystBot".to_owned()))
      // set a header
      .header(Connection::close())
      // let 'er go!
      .send().unwrap();
  //.body("{\"body\": \"Test comment from Catalyst!\"}")

  // Read the Response.
  let mut body = String::new();
  res.read_to_string(&mut body).unwrap();

  println!("Response: {}", body);
  println!("Response: {:?}", res);
}
