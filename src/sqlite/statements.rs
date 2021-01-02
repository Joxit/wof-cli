pub const TABLE_GEOJSON: &'static str = r#"CREATE TABLE IF NOT EXISTS geojson (
	id INTEGER NOT NULL,
	body TEXT,
	source TEXT,
	is_alt BOOLEAN,
	lastmodified INTEGER
);"#;

pub const INDEXES_GEOJSON: &'static str = r#"CREATE UNIQUE INDEX IF NOT EXISTS geojson_by_id ON geojson (id, source);
CREATE INDEX IF NOT EXISTS geojson_by_alt ON geojson (id, is_alt);
CREATE INDEX IF NOT EXISTS geojson_by_lastmod ON geojson (lastmodified);"#;

pub const INSERT_GEOJSON: &'static str = r#"
INSERT OR REPLACE INTO geojson (id, body, source, is_alt, lastmodified) VALUES (?, ?, ?, ?, ?)
"#;

pub const TABLE_SPR: &'static str = r#"CREATE TABLE IF NOT EXISTS spr (
	id INTEGER NOT NULL PRIMARY KEY,
	parent_id INTEGER,
	name TEXT,
	placetype TEXT,
	country TEXT,
	repo TEXT,
	latitude REAL,
	longitude REAL,
	min_latitude REAL,
	min_longitude REAL,
	max_latitude REAL,
	max_longitude REAL,
	is_current INTEGER,
	is_deprecated INTEGER,
	is_ceased INTEGER,
	is_superseded INTEGER,
	is_superseding INTEGER,
	superseded_by TEXT,
	supersedes TEXT,
	lastmodified INTEGER
);"#;

pub const INDEXES_SPR: &'static str = r#"CREATE INDEX IF NOT EXISTS spr_by_lastmod ON spr (lastmodified);
CREATE INDEX IF NOT EXISTS spr_by_parent ON spr (parent_id, is_current, lastmodified);
CREATE INDEX IF NOT EXISTS spr_by_placetype ON spr (placetype, is_current, lastmodified);
CREATE INDEX IF NOT EXISTS spr_by_country ON spr (country, placetype, is_current, lastmodified);
CREATE INDEX IF NOT EXISTS spr_by_name ON spr (name, placetype, is_current, lastmodified);
CREATE INDEX IF NOT EXISTS spr_by_centroid ON spr (latitude, longitude, is_current, lastmodified);
CREATE INDEX IF NOT EXISTS spr_by_bbox ON spr (min_latitude, min_longitude, max_latitude, max_longitude, placetype, is_current, lastmodified);
CREATE INDEX IF NOT EXISTS spr_by_repo ON spr (repo, lastmodified);
CREATE INDEX IF NOT EXISTS spr_by_current ON spr (is_current, lastmodified);
CREATE INDEX IF NOT EXISTS spr_by_deprecated ON spr (is_deprecated, lastmodified);
CREATE INDEX IF NOT EXISTS spr_by_ceased ON spr (is_ceased, lastmodified);
CREATE INDEX IF NOT EXISTS spr_by_superseded ON spr (is_superseded, lastmodified);
CREATE INDEX IF NOT EXISTS spr_by_superseding ON spr (is_superseding, lastmodified);
CREATE INDEX IF NOT EXISTS spr_obsolete ON spr (is_deprecated, is_superseded);"#;

pub const INSERT_SPR: &'static str = r#"
INSERT OR REPLACE INTO spr (
  id, parent_id, name, placetype, country, repo, latitude, longitude,
  min_latitude, min_longitude, max_latitude, max_longitude,
  is_current, is_deprecated, is_ceased, is_superseded, is_superseding,
  superseded_by, supersedes, lastmodified
)
VALUES (
  ?, ?, ?, ?, ?, ?, ?, ?,
  ?, ?, ?, ?,
  ?, ?, ?, ?, ?,
  ?, ?, ?
)
"#;

pub const TABLE_NAMES: &'static str = r#"CREATE TABLE IF NOT EXISTS names (
   id INTEGER NOT NULL,
   placetype TEXT,
   country TEXT,
   language TEXT,
   extlang TEXT,
   script TEXT,
   region TEXT,
   variant TEXT,
   extension TEXT,
   privateuse TEXT,
   name TEXT,
   lastmodified INTEGER
);"#;

pub const INSERT_NAMES: &'static str = r#"
INSERT OR REPLACE INTO names (
   id, placetype, country, language, extlang, script,
   region, variant, extension, privateuse, name, lastmodified
) VALUES (
	?, ?, ?, ?, ?, ?,
	?, ?, ?, ?, ?, ?
);"#;

pub const INDEXES_NAMES: &'static str = r#"CREATE INDEX IF NOT EXISTS names_by_lastmod ON names (lastmodified);
CREATE INDEX IF NOT EXISTS names_by_country ON names (country,privateuse,placetype);
CREATE INDEX IF NOT EXISTS names_by_language ON names (language,privateuse,placetype);
CREATE INDEX IF NOT EXISTS names_by_placetype ON names (placetype,country,privateuse);
CREATE INDEX IF NOT EXISTS names_by_name ON names (name, placetype, country);
CREATE INDEX IF NOT EXISTS names_by_name_private ON names (name, privateuse, placetype, country);
CREATE INDEX IF NOT EXISTS names_by_wofid ON names (id);"#;

pub const TABLE_ANCESTORS: &'static str = r#"CREATE TABLE IF NOT EXISTS ancestors (
    id INTEGER NOT NULL,
    ancestor_id INTEGER NOT NULL,
    ancestor_placetype TEXT,
    lastmodified INTEGER
);"#;

pub const INDEXES_ANCESTORS: &'static str = r#"CREATE INDEX IF NOT EXISTS ancestors_by_id ON ancestors (id,ancestor_placetype,lastmodified);
CREATE INDEX IF NOT EXISTS ancestors_by_ancestor ON ancestors (ancestor_id,ancestor_placetype,lastmodified);
CREATE INDEX IF NOT EXISTS ancestors_by_lastmod ON ancestors (lastmodified);"#;

pub const INSERT_ANCESTORS: &'static str = r#"
INSERT OR REPLACE INTO ancestors (
   id, ancestor_id, ancestor_placetype, lastmodified
) VALUES (
	?, ?, ?, ?
);"#;

pub const TABLE_CONCORDANCES: &'static str = r#"CREATE TABLE IF NOT EXISTS concordances (
	id INTEGER NOT NULL,
	other_id INTEGER NOT NULL,
	other_source TEXT,
	lastmodified INTEGER
);"#;

pub const INDEXES_CONCORDANCES: &'static str = r#"CREATE INDEX IF NOT EXISTS concordances_by_id ON concordances (id,lastmodified);
CREATE INDEX IF NOT EXISTS concordances_by_other_id ON concordances (other_source,other_id);
CREATE INDEX IF NOT EXISTS concordances_by_other_lastmod ON concordances (other_source,other_id,lastmodified);
CREATE INDEX IF NOT EXISTS concordances_by_lastmod ON concordances (lastmodified);"#;

pub const INSERT_CONCORDANCES: &'static str = r#"
INSERT OR REPLACE INTO concordances (
   id, other_id, other_source, lastmodified
) VALUES (
	?, ?, ?, ?
);"#;

// Tweaks for perf:
// https://www.sqlite.org/pragma.html
// https://blog.devart.com/increasing-sqlite-performance.html
pub const PRAGMA: &'static str = r#"PRAGMA JOURNAL_MODE=OFF;
PRAGMA SYNCHRONOUS=OFF;
PRAGMA LOCKING_MODE=EXCLUSIVE;
PRAGMA PAGE_SIZE=4096;
PRAGMA CACHE_SIZE=1000000;
PRAGMA TEMP_STORE=MEMORY;"#;

pub const SELECT_ALL_IDS: &'static str = "SELECT id FROM geojson";

pub const SELECT_ALL_GEOJSONS: &'static str = "SELECT body FROM geojson";
