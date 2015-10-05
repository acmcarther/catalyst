use github_v3::types::{
  GitTm,
  Sha
};

pub type BranchName = String;

#[derive(Debug, PartialEq, Clone)]
pub enum BuildStatus {
  Passed,
  Failed,
  Pending
}

#[derive(Debug, PartialEq, Clone)]
pub struct BuildStep {
  pub name: String,
  pub status: BuildStatus,
  pub output_url: Option<String>,
  pub bash_command: Option<String>
}

#[derive(Debug, PartialEq, Clone)]
pub struct Build {
  pub status: BuildStatus,
  pub branch: BranchName,
  pub started_at: Option<GitTm>,
  pub finished_at: Option<GitTm>,
  pub commit: Sha,
  pub commit_message: String,
  pub steps: Vec<BuildStep>
}


pub trait CIIntegration {
  fn build_start();
  fn build_success();
  fn build_failure();
}
