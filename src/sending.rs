pub use self::sending::{
  post_comment
};

mod sending {
  use hyper::Client;
  use hyper::header::{Authorization, Connection, UserAgent};
  use hyper::client::response::Response;
  use std::io::Read;

  pub fn post_comment(auth_header: Authorization<String>, repo_owner: String, repo_name: String, issue_number: u32, comment: String, client: &Client) -> String {
    let comments_url = "https://api.github.com/repos/".to_owned() + &repo_owner + "/" + &repo_name + "/issues" + &(issue_number.to_string()) + "/comments";
    let message = "{\"body\": \"".to_owned() + &comment + "\"}";

    let mut res = client.post(&comments_url)
        // set a header
        .header(auth_header)
        // set a header
        .header(UserAgent("CatalystBot".to_owned()))
        // set a header
        .header(Connection::close())
        // set a body
        .body(&message)
        // let 'er go!
        .send().unwrap();

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    body
  }
}
