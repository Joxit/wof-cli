use crate::Wof;
use clap::{CommandFactory, Parser};
use clap_complete::{generate, Shell};

#[derive(Debug, Parser)]
pub enum Completion {
  /// Generates a .bash completion file for the Bourne Again SHell (BASH)
  /// Save the output in `/etc/bash_completion.d/wof` or `~/.local/share/bash-completion/completions/wof`
  #[command(name = "bash")]
  Bash,
  /// Generates a .fish completion file for the Friendly Interactive SHell (fish)
  #[command(name = "fish")]
  Fish,
  /// Generates a completion file for the Z SHell (ZSH)
  #[command(name = "zsh")]
  Zsh,
  /// Generates a completion file for Elvish
  #[command(name = "elvish")]
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
    let mut cli = Wof::command();
    let bin_name = cli.get_name().to_string();
    generate(shell, &mut cli, &bin_name, &mut std::io::stdout());
  }
}
