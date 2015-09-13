use iron::mime::Mime;

use iron::{Request, Response, IronResult, status};

use router::Router;

fn handle_api(req: &mut Request) -> IronResult<Response> {
  let path = req.url.path.iter().fold("".to_string(), |acc, item| acc + "/" + &item);
  let response_str = "unknown api endpoint ".to_string() + &path;
  Ok(Response::with((status::Ok, response_str)))
}

fn handle_api_get_users(_: &mut Request) -> IronResult<Response> {
  let content_type = "application/json".parse::<Mime>().unwrap();
  let response = Response::with((
      content_type,
      status::Ok,
      "{\"users\": [\"bob\"]}"
  ));
  Ok(response)
}

fn handle_api_post_users(_: &mut Request) -> IronResult<Response> {
  let content_type = "application/json".parse::<Mime>().unwrap();
  let response = Response::with((
      content_type,
      status::Ok,
      "{\"status\": \"ok\"}"
  ));
  Ok(response)
}

pub fn get_api_handler() -> Router {
  let mut api_router = Router::new();

  api_router.get("/users", handle_api_get_users);
  api_router.post("/users", handle_api_post_users);
  api_router.get("*", handle_api);
  api_router
}
