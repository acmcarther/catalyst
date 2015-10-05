use iron::{Chain, Request, Response, IronResult, BeforeMiddleware, AfterMiddleware, typemap, status};

use router::Router;

use std::io::Read;
use std::sync::Mutex;
use std::sync::mpsc::Sender;

use rustc_serialize::{json};

use continuous_integrator::types::Build;
use continuous_integrator::circle_integration::CircleBuild;

struct Deserialize;

struct IsCircleBuild;
impl typemap::Key for IsCircleBuild { type Value = CircleBuild; }

impl BeforeMiddleware for Deserialize {
  fn before(&self, req: &mut Request) -> IronResult<()> {
    let mut payload = String::new();
    req.body.read_to_string(&mut payload).unwrap();
    req.extensions.insert::<IsCircleBuild>(json::decode(&payload).unwrap());
    Ok(())
  }
}

struct DeliverActionables {
  build_tx: Mutex<Sender<Build>>,
}

impl AfterMiddleware for DeliverActionables {
  fn after(&self, req: &mut Request, response: Response) -> IronResult<Response> {
    let possible_payload = req.extensions.remove::<IsCircleBuild>();
    possible_payload.map(|payload: CircleBuild| self.build_tx.lock().map(|sender| sender.send(Build::from(payload))));
    Ok(response)
  }
}

fn handle_webhooks(_: &mut Request) -> IronResult<Response> {
  Ok(Response::with((status::Accepted, "{\"body\":\"Build (possibly) Received\"}")))
}

pub fn get_webhook_handler(
  build_tx: Sender<Build>
  ) -> Router {

  let deliverer = DeliverActionables {
    build_tx: Mutex::new(build_tx),
  };

  let mut webhook_chain = Chain::new(handle_webhooks);
  webhook_chain.link_before(Deserialize);
  webhook_chain.link_after(deliverer);

  let mut webhook_router = Router::new();

  webhook_router.post("/", webhook_chain);
  webhook_router
}
