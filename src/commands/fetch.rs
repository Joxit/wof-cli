use crate::std::download_tar_gz_strip;
use crate::std::ResultExit;
use std::path::Path;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Fetch {
  /// Ouput directory to download WOF documents
  #[structopt(short = "o", long = "out", default_value = ".")]
  pub out: String,
  /// Should download postalcodes repositories
  #[structopt(long = "postalcode", possible_values = &["true", "false"])]
  pub postalcode: Option<bool>,
  /// Should download admin repositories, default true
  #[structopt(long = "admin", possible_values = &["true", "false"])]
  pub admin: Option<bool>,
  /// Two letters country code to download.
  #[structopt(short = "c", long = "country")]
  pub countries: Vec<String>,
}

impl Fetch {
  pub fn exec(&self) {
    let download_dest = Path::new(&self.out);
    for country in &self.countries {
      if self.admin.unwrap_or(true) {
        let url = Fetch::get_url(country.to_string(), "admin");
        download_tar_gz_strip(url.to_string(), download_dest.to_path_buf(), 1)
          .expect_exit(format!("Something goes wrong when downloading `{}`", url).as_str());
      }
      if self.postalcode.unwrap_or(false) {
        let url = Fetch::get_url(country.to_string(), "postalcode");
        download_tar_gz_strip(url.to_string(), download_dest.to_path_buf(), 1)
          .expect_exit(format!("Something goes wrong when downloading `{}`", url).as_str());
      }
    }
  }

  fn get_url(country: String, r#type: &'static str) -> String {
    format!(
      "https://github.com/whosonfirst-data/whosonfirst-data-{}-{}/archive/master.tar.gz",
      r#type, country
    )
  }
}
