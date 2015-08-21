pub use self::listening::{
  start_listener
};

mod listening {
  use std::thread;
  use hyper::Client;
  use hyper::header::{Authorization, Connection, UserAgent};
  use std::io::Read;

  pub fn start_listener(token: String, repo_owner: String, repo_name: String) {
    loop {
      let mut client = Client::new();

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

      thread::sleep_ms(10000);
    }
  }
}
