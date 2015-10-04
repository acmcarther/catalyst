use github_v3::types::{
  GitTm,
  Sha
};

pub type BranchName = String;

pub enum BuildStatus {
  Passed,
  Failed,
  Pending
}

pub enum BuildType {
  Push,
  PullRequest(u32)
}

pub struct Build {
  status: BuildStatus,
  branch: BranchName,
  started_at: Option<GitTm>,
  finished_at: Option<GitTm>,
  commit: Sha,
  commit_message: String,
}


pub trait CIIntegration {
  fn build_start();
  fn build_success();
  fn build_failure();
}
