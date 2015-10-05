use rustc_serialize::{
  Decodable,
  Decoder,
  Encodable,
  Encoder,
};
use github_v3::types::{
  GitTm,
  Sha
};

use continuous_integrator::types::{
  Build,
  BuildStatus,
  BuildStep,
};

use std::convert::From;

#[derive(Debug, PartialEq, Clone)]
pub enum CircleLifecycle {
  Queued,
  Scheduled,
  NotRun,
  NotRunning,
  Running,
  Finished
}

custom_enum_decode_encode!(
  CircleLifecycle [
    "queued" <=> [CircleLifecycle::Queued],
    "scheduled" <=> [CircleLifecycle::Scheduled],
    "not_run" <=> [CircleLifecycle::NotRun],
    "not_running" <=> [CircleLifecycle::NotRunning],
    "running" <=> [CircleLifecycle::Running],
    "finished" <=> [CircleLifecycle::Finished],
  ]
);

#[derive(Debug, PartialEq, Clone)]
pub enum CircleOutcome {
  Canceled,
  InfrastructureFail,
  Timedout,
  Failed,
  NoTests,
  Success
}

custom_enum_decode_encode!(
  CircleOutcome [
    "canceled" <=> [CircleOutcome::Canceled],
    "infrastructure_fail" <=> [CircleOutcome::InfrastructureFail],
    "timedout" <=> [CircleOutcome::Timedout],
    "failed" <=> [CircleOutcome::Failed],
    "no_tests" <=> [CircleOutcome::NoTests],
    "success" <=> [CircleOutcome::Success],
  ]
);

#[derive(Debug, PartialEq, Clone)]
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

custom_enum_decode_encode!(
  CircleStatus [
    "retried" <=> [CircleStatus::Retried],
    "canceled" <=> [CircleStatus::Canceled],
    "infrastructure_fail" <=> [CircleStatus::InfrastructureFail],
    "timedout" <=> [CircleStatus::Timedout],
    "not_run" <=> [CircleStatus::NotRun],
    "running" <=> [CircleStatus::Running],
    "failed" <=> [CircleStatus::Failed],
    "queued" <=> [CircleStatus::Queued],
    "scheduled" <=> [CircleStatus::Scheduled],
    "not_running" <=> [CircleStatus::NotRunning],
    "no_tests" <=> [CircleStatus::NoTests],
    "fixed" <=> [CircleStatus::Fixed],
    "success" <=> [CircleStatus::Success],
  ]
);


#[derive(RustcDecodable, RustcEncodable, Debug, PartialEq, Clone)]
pub struct CircleStep {
  pub name: String,
  pub actions: Vec<CircleActions>
}

#[derive(RustcDecodable, RustcEncodable, Debug, PartialEq, Clone)]
pub struct CircleActions {
  pub bash_command: Option<String>,
  pub status: CircleStatus,
  pub output_url: Option<String>,
  // TODO: Many fields omitted
}

#[derive(RustcDecodable, RustcEncodable, Debug, PartialEq, Clone)]
pub struct CirclePayload {
  pub vcs_url: String,
  pub build_url: String,
  pub branch: String,
  pub vcs_revision: Sha,
  pub committer_name: String,
  pub committer_email: String,
  pub subject: String,
  pub body: String,
  pub why: String,
  // TODO: Find why these parse slow
  //pub queued_at: GitTm,
  //pub start_time: Option<GitTm>,
  //pub stop_time: Option<GitTm>,
  pub build_time_millis: u32,
  pub username: String,
  pub reponame: String,
  pub lifecycle: CircleLifecycle,
  pub outcome: CircleOutcome,
  pub retry_of: Option<u32>,
  pub steps: Vec<CircleStep>
}

#[derive(RustcDecodable, RustcEncodable, Debug, PartialEq, Clone)]
pub struct CircleBuild {
  payload: CirclePayload
}

// NOTE: this conversion uses the first action, as CircleCI seems to consistently just have one
// action per build step (Oct 4 2015)
impl From<CircleStep> for BuildStep {
  fn from(mut step: CircleStep) -> BuildStep {
    let relevant_action = step.actions.remove(0);
    BuildStep {
      name: step.name,
      status: BuildStatus::from(relevant_action.status),
      output_url: relevant_action.output_url,
      bash_command: relevant_action.bash_command,
    }
  }
}

impl From<CircleStatus> for BuildStatus {
  fn from(status: CircleStatus) -> BuildStatus {
    match status {
      CircleStatus::Retried => BuildStatus::Pending,
      CircleStatus::Canceled => BuildStatus::Failed,
      CircleStatus::InfrastructureFail => BuildStatus::Failed,
      CircleStatus::Timedout => BuildStatus::Failed,
      CircleStatus::NotRun => BuildStatus::Failed,
      CircleStatus::Running => BuildStatus::Pending,
      CircleStatus::Failed => BuildStatus::Failed,
      CircleStatus::Queued => BuildStatus::Pending,
      CircleStatus::Scheduled => BuildStatus::Pending,
      CircleStatus::NotRunning => BuildStatus::Failed,
      CircleStatus::NoTests => BuildStatus::Passed,
      CircleStatus::Fixed => BuildStatus::Passed,
      CircleStatus::Success => BuildStatus::Passed,
    }
  }
}

impl From<(CircleLifecycle, CircleOutcome)> for BuildStatus {
  fn from(status: (CircleLifecycle, CircleOutcome)) -> BuildStatus {
    let (lifecycle, outcome) = status;
    match lifecycle {
      CircleLifecycle::Queued => BuildStatus::Pending,
      CircleLifecycle::Scheduled => BuildStatus::Pending,
      CircleLifecycle::NotRun => BuildStatus::Failed,
      CircleLifecycle::NotRunning => BuildStatus::Failed,
      CircleLifecycle::Running => BuildStatus::Pending,
      CircleLifecycle::Finished => {
        match outcome {
          CircleOutcome::Canceled => BuildStatus::Failed,
          CircleOutcome::InfrastructureFail => BuildStatus::Failed,
          CircleOutcome::Timedout => BuildStatus::Failed,
          CircleOutcome::Failed => BuildStatus::Failed,
          CircleOutcome::NoTests => BuildStatus::Passed,
          CircleOutcome::Success => BuildStatus::Passed,
        }
      }
    }
  }
}
impl From<CircleBuild> for Build {
  fn from(build: CircleBuild) -> Build {
    let payload = build.payload;
    Build {
      status: BuildStatus::from((payload.lifecycle, payload.outcome)),
      branch: payload.branch,
      commit: payload.vcs_revision,
      commit_message: payload.subject,
      started_at: None,//payload.start_time,
      finished_at: None,//payload.stop_time,
      steps: payload.steps.into_iter().map(|step| BuildStep::from(step)).collect()
    }
  }
}
