use iron::mime::Mime;

use iron::{Iron, Request, Response, IronResult, status};

use router::Router;
use mount::Mount;
use staticfile::Static;

use std::io::Read;
use std::sync::mpsc::Sender;
use std::fs::File;
use std::path::Path;

use std::io::Error as IoError;

use client_api;
use webhooks;

use types::HandledGithubEvents;
use continuous_integrator::types::Build;

fn serve_file(file_path: &str) -> Result<String, IoError> {
  let mut s = String::new();
  File::open(file_path)
    .and_then(|mut file| file.read_to_string(&mut s))
    .map(|_| s)
}

fn handle_root(_: &mut Request) -> IronResult<Response> {
  let content_type = "text/html".parse::<Mime>().unwrap();
  let response = Response::with((
      content_type,
      status::Ok,
      serve_file("./client/index.html").unwrap(),
  ));
  Ok(response)
}

pub fn spawn_listener(
  event_tx: Sender<HandledGithubEvents>,
  build_tx: Sender<Build>,
  ) -> Iron<Mount> {

  let mut router = Router::new();
  let mut mount = Mount::new();

  mount.mount("/api_v1/", client_api::get_api_handler());
  mount.mount("/github_webhooks", webhooks::github_webhook_handler(event_tx));
  mount.mount("/circle_webhooks", webhooks::circle_webhook_handler(build_tx));
  mount.mount("/assets/", Static::new(Path::new("client/dist/")));
  router.get("/", handle_root);
  mount.mount("/", router);

  Iron::new(mount)
}
