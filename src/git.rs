use crate::std::ResultExit;
use git2::{DiffFormat, ObjectType, Repository};
use std::path::PathBuf;
use structopt::StructOpt;

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

  pub fn repository(&self) -> Repository {
    Repository::discover(self.workdir.as_path())
      .expect("This is not a git repository. Should not happen.")
  }

  pub fn exec(&self, commit: String) {
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

    repo
      .diff_tree_to_tree(Some(&tree_src), tree_dst.as_tree(), None)
      .expect("Can't create the diff between source and destination")
      .print(DiffFormat::NameOnly, |_delta, _hunk, line| {
        let path = self
          .workdir
          .as_path()
          .clone()
          .join(std::str::from_utf8(line.content()).expect("WOF Elements should be utf-8."));
        println!("{:?}", path);
        true
      })
      .expect_exit("Can't create the diff for this commit");
  }
}
