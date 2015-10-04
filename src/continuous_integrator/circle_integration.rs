use github_v3::types::{
  GitTm,
  Sha
};

use continuous_integrator::types::{
  Build,
  BuildStatus,
  BuildType
};

use std::convert::From;

// TODO: Custom encoding/decoding
pub enum CircleLifecycle {
  Queued,
  Scheduled,
  NotRun,
  NotRunning,
  Running,
  Finished
}

// TODO: Custom encoding/decoding
pub enum CircleOutcome {
  Canceled,
  InfrastructureFail,
  Timedout,
  Failed,
  NoTests,
  Success
}

// TODO: Custom encoding/decoding
pub enum CircleStatus {
  Retried,
  Canceled,
  InfrastructureFail,
  Timedout,
  NotRun,
  Running,
  Failed,
  Queued,
  Scheduled,
  NotRunning,
  NoTests,
  Fixed,
  Success
}

pub struct CircleSteps {
  name: String,
  actions: Vec<CircleActions>
}

pub struct CircleActions {
  bash_command: Option<String>,
  run_time_millis: u32,
  start_time: GitTm,
  end_time: Option<GitTm>,
  exit_code: Option<u32>,
  index: u32, // TODO: Identify this field's purpose?
  //type: CircleActionType // TODO: Custom De/Serialization
  step: Option<u32>, // TODO: Identify why this field is optional
  // TODO: Identify weirdly optional fields
  name: Option<String>,
  source: Option<String>, // TODO: This can apparently be "inference", Will get a dedicated types when options are enumerated
  failed: Option<bool>, // TODO: Identify why this is an optional bool, what the heck CircleCI. Optional Bool. Optional Bool. Thats not how optional works
  messages: Option<Vec<String>> // TODO: OPTIONAL VEC, WHAT THE HECK

}

pub struct CirclePayload {
  vcs_url: String,
  build_url: String,
  branch: String,
  vcs_revision: Sha,
  committer_name: String,
  committer_email: String,
  subject: String,
  body: String,
  why: String,
  dont_build: String,
  queued_at: GitTm,
  start_time: GitTm,
  stop_time: GitTm,
  build_time_millis: u32,
  username: String,
  reponame: String,
  lifecycle: CircleLifecycle,
  outcome: CircleOutcome,
  retry_of: Option<u32>,
  steps: Vec<CircleSteps>
}

pub struct CircleBuild {
  payload: CirclePayload
}

/*
impl From<CircleBuild> for Build {
  fn from(build: CircleBuild) -> Build {
    Build {
    }
  }
}
*/
