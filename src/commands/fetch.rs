use crate::commands::{download_tar_gz_stream_geojson, download_tar_gz_strip, output_pipe};
use crate::utils;
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
  /// Two letters country code to download. No values will download all repositories.
  pub countries: Vec<String>,
}

impl Fetch {
  pub fn exec(&self) {
    let download_dest = Path::new(&self.out);
    let all_countries = utils::get_available_country_codes();
    let countries = if self.countries.len() > 0 {
      &self.countries
    } else {
      &all_countries
    };

    let stdout = output_pipe();

    for country in countries {
      if self.admin.unwrap_or(true) {
        let url = Fetch::get_url(country.to_string(), "admin");
        if let Err(e) = if stdout {
          download_tar_gz_stream_geojson(url.to_string())
        } else {
          download_tar_gz_strip(url.to_string(), download_dest.to_path_buf(), 1)
        } {
          eprintln!("Something goes wrong when downloading `{}`: {}", url, e);
        }
      }
      if self.postalcode.unwrap_or(false) {
        let url = Fetch::get_url(country.to_string(), "postalcode");
        if let Err(e) = download_tar_gz_strip(url.to_string(), download_dest.to_path_buf(), 1) {
          eprintln!("Something goes wrong when downloading `{}`: {}", url, e);
        }
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
