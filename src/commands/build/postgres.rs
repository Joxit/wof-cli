use crate::postgres;
use crate::utils::ResultExit;
use clap::Parser;
use log::info;

#[derive(Debug, Parser)]
pub struct Postgres {
  /// WOF data directories to import
  #[arg(default_value = ".")]
  pub directories: Vec<String>,
  /// The IP or hostname of the postgreSQL database.
  #[arg(long = "host", default_value = "127.0.0.1", env = "WOF_PG_HOST")]
  pub host: String,
  /// The postgreSQL user name to use.
  #[arg(
    short = 'u',
    long = "user",
    default_value = "wof",
    env = "WOF_PG_USERNAME"
  )]
  pub user: String,
  /// The postgreSQL database name to use.
  #[arg(
    short = 'd',
    long = "dbname",
    default_value = "gis",
    env = "WOF_PG_DBNAME"
  )]
  pub dbname: String,
  /// The postgreSQL database port to use.
  #[arg(
    short = 'p',
    long = "port",
    default_value = "5432",
    env = "WOF_PG_DBNAME"
  )]
  pub port: u16,
  /// The postgreSQL database port to use.
  #[arg(short = 'W', long = "password", env = "WOF_PG_PASSWORD")]
  pub password: Option<String>,
  /// The SIRID to use for geometry storage. Default value is 4326, common usage is also 3857.
  #[arg(
    short = 's',
    long = "srid",
    default_value = "4326",
    env = "WOF_PG_SRID"
  )]
  pub srid: i32,
  /// Don't insert deprecated features.
  #[arg(long = "no-deprecated")]
  pub no_deprecated: bool,
  /// Display timings during the build process, implies verbose.
  #[arg(long = "timings")]
  pub timings: bool,
  /// Activate verbose mode.
  #[arg(short = 'v', long = "verbose")]
  pub verbose: bool,
}

impl Postgres {
  pub fn exec(&self) {
    crate::utils::logger::set_verbose(self.verbose || self.timings, "wof::build::postgres")
      .expect_exit("Can't init logger.");
    let mut config = postgres::Config::new();

    config.dbname(&self.dbname);
    config.user(&self.user);
    config.host(&self.host);
    config.port(self.port);
    if let Some(password) = &self.password {
      config.password(&password);
    }

    info!("Connecting to database: `{:?}`", config.get_hosts());
    let mut postgres =
      postgres::Postgres::new(config, Some(self.srid)).expect_exit("Can't open the database");

    info!("Creating tables and indexes.");
    postgres.create_tables().expect_exit("Can't create tables");

    crate::commands::build::build_database(&self.directories, self.timings, &mut |buffer, file| {
      if let Some(buffer) = buffer {
        postgres.add_string(buffer)
      } else if let Some(file) = file {
        postgres.add_file(file)
      } else {
        Ok(())
      }
    });
  }
}
