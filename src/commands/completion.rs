use crate::ApplicationArguments;
use structopt::clap::Shell;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Completion {
  /// Generates a .bash completion file for the Bourne Again SHell (BASH)
  #[structopt(name = "bash")]
  Bash,
  /// Generates a .fish completion file for the Friendly Interactive SHell (fish)
  #[structopt(name = "fish")]
  Fish,
  /// Generates a completion file for the Z SHell (ZSH)
  #[structopt(name = "zsh")]
  Zsh,
  /// Generates a completion file for Elvish
  #[structopt(name = "elvish")]
  Elvish,
}

impl Completion {
  pub fn exec(&self) {
    let shell = match self {
      Completion::Bash => Shell::Bash,
      Completion::Fish => Shell::Fish,
      Completion::Zsh => Shell::Zsh,
      Completion::Elvish => Shell::Elvish,
    };
    ApplicationArguments::clap().gen_completions_to("wof", shell, &mut std::io::stdout());
  }
}
