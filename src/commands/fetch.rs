use crate::commands::{download_tar_gz_stream_geojson, download_tar_gz_strip, output_pipe};
use crate::utils;
use crate::utils::ResultExit;
use clap::builder::PossibleValuesParser;
use clap::Parser;
use log::{error, info};
use std::path::Path;
use std::time::SystemTime;

#[derive(Debug, Parser)]
pub struct Fetch {
  /// Ouput directory to download WOF documents
  #[arg(short = 'o', long = "out", default_value = ".")]
  pub out: String,
  /// Should download postalcodes repositories
  #[arg(long = "postalcode", default_value = "false")]
  pub postalcode: Option<bool>,
  /// Should download admin repositories, default true
  #[arg(long = "admin", default_value = "true")]
  pub admin: Option<bool>,
  /// Two letters country code to download. No values will download all repositories.
  pub countries: Vec<String>,
  /// Display timings during the download process, implies verbose.
  #[arg(long = "timings")]
  pub timings: bool,
  /// Activate verbose mode.
  #[arg(short = 'v', long = "verbose")]
  pub verbose: bool,
}

impl Fetch {
  pub fn exec(&self) {
    let download_dest = Path::new(&self.out);
    let all_countries = utils::get_available_country_codes();
    crate::utils::logger::set_verbose(self.verbose || self.timings, "wof::fetch")
      .expect_exit("Can't init logger.");
    let countries = if self.countries.len() > 0 {
      info!("Will download {} countries.", self.countries.len());
      &self.countries
    } else {
      info!("Will download all countries.");
      &all_countries
    };

    let stdout = output_pipe();

    for country in countries {
      if self.admin.unwrap_or(true) {
        self.fetch(country.to_string(), stdout, "admin", download_dest);
      }
      if self.postalcode.unwrap_or(false) {
        self.fetch(country.to_string(), stdout, "postalcode", download_dest);
      }
    }
  }

  #[inline]
  fn fetch(&self, country: String, stdout: bool, r#type: &'static str, download_dest: &Path) {
    let url = Fetch::get_url(country.to_string(), r#type);
    let start = SystemTime::now();
    if let Err(e) = if stdout {
      info!("Fetching {} to stdout", url);
      download_tar_gz_stream_geojson(url.to_string())
    } else {
      info!("Fetching {} to {}", url, download_dest.display());
      download_tar_gz_strip(url.to_string(), download_dest.to_path_buf(), 1)
    } {
      error!("Something goes wrong when downloading `{}`: {}", url, e);
    } else if self.timings {
      info!(
        "Fetch of {} finished successfully in {:?}.",
        url,
        start.elapsed().unwrap()
      );
    }
  }

  fn get_url(country: String, r#type: &'static str) -> String {
    format!(
      "https://github.com/whosonfirst-data/whosonfirst-data-{}-{}/archive/master.tar.gz",
      r#type, country
    )
  }
}
