use crate::std::ResultExit;
use git2::{DiffFormat, DiffLine, ObjectType, Repository};
use std::path::PathBuf;
use std::vec::Vec;

pub struct Git {
  workdir: PathBuf,
}

impl Git {
  pub fn new() -> Git {
    return Git {
      workdir: Repository::discover(".")
        .expect_exit("This is not a git repository. Make sure to be in a git folder.")
        .workdir()
        .expect("Git working directory not found. Should not happen.")
        .to_path_buf(),
    };
  }

  pub fn data_dir(&self) -> String {
    String::from(
      self
        .workdir
        .join("data")
        .to_str()
        .expect("Can't convert the workdir to str."),
    )
  }

  pub fn repository(&self) -> Repository {
    Repository::discover(self.workdir.as_path())
      .expect("This is not a git repository. Should not happen.")
  }

  pub fn get_changes_from_commit(&self, commit: &String) -> Vec<PathBuf> {
    let repo = self.repository();
    let obj_dst = repo
      .revparse_single(commit.as_str())
      .expect_exit(format!("Commit/branch {} not found", commit).as_str());
    let obj_src = obj_dst
      .peel_to_commit()
      .expect("Can't convert the object to a commit. Should not happen.")
      .parent(0)
      .expect_exit(format!("Can't the parent object of {}", commit).as_str());

    let tree_src = obj_src.tree().expect("Can't get the source tree.");
    let tree_dst = obj_dst
      .peel(ObjectType::Tree)
      .expect("Can't get the destination tree.");
    let mut paths: Vec<PathBuf> = Vec::new();
    repo
      .diff_tree_to_tree(Some(&tree_src), tree_dst.as_tree(), None)
      .expect("Can't create the diff between source and destination")
      .print(DiffFormat::NameOnly, |_delta, _hunk, line| {
        paths.push(self.diff_line_to_real_path(line));
        true
      })
      .expect_exit("Can't create the diff for this commit");
    paths
  }

  pub fn get_changes_from_stagged(&self) -> Vec<PathBuf> {
    let mut paths: Vec<PathBuf> = Vec::new();
    let repo = self.repository();
    let tree = repo
      .revparse_single("HEAD")
      .expect_exit("HEAD not found in your git tree")
      .peel(ObjectType::Tree)
      .expect("Can't get the HEAD tree.");

    repo
      .diff_tree_to_index(tree.as_tree(), None, None)
      .expect("Can't create the diff for stagged elements")
      .print(DiffFormat::NameOnly, |_delta, _hunk, line| {
        paths.push(self.diff_line_to_real_path(line));
        true
      })
      .expect_exit("Can't create the stagged diff");
    paths
  }

  fn diff_line_to_real_path(&self, line: DiffLine) -> PathBuf {
    self.workdir.as_path().clone().join(
      std::str::from_utf8(line.content())
        .expect("WOF Elements should be utf-8.")
        .trim(),
    )
  }
}
