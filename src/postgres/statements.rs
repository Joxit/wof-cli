pub const TABLE_GEOMETRIES: &'static str = r#"CREATE TABLE IF NOT EXISTS wof_geometries (
  id INTEGER NOT NULL,
  geometry public.geometry(Geometry, ${srid}),
  source TEXT,
  placetype TEXT,
  name TEXT,
  country TEXT,
  properties JSONB,
  is_alt BOOLEAN,
  lastmodified INTEGER,

  CONSTRAINT wof_geometries_pkey PRIMARY KEY (id, source)
);"#;

pub const INDEXES_GEOMETRIES: &'static str = r#"CREATE INDEX IF NOT EXISTS wof_geometries_geom ON public.wof_geometries USING gist (geometry);
CREATE INDEX IF NOT EXISTS wof_geometries_geom_geohash ON public.wof_geometries USING btree (public.st_geohash(public.st_transform(public.st_setsrid((public.box2d(geometry))::public.geometry, ${srid}), 4326)));
ALTER TABLE public.wof_geometries CLUSTER ON wof_geometries_geom_geohash;
"#;

pub const INSERT_GEOMETRIES: &'static str = r#"
INSERT INTO wof_geometries (id, geometry, source, properties, is_alt, lastmodified, placetype, name, country) VALUES ($1, ST_Transform(ST_SetSRID(ST_GeomFromGeoJSON($2), 4326), $7::integer), $3, ($4)::text::jsonb, $5, $6, $8, $9, $10)
ON CONFLICT ON CONSTRAINT wof_geometries_pkey
DO UPDATE SET geometry = excluded.geometry, properties = excluded.properties, is_alt = excluded.is_alt, lastmodified = excluded.lastmodified
WHERE wof_geometries.id = excluded.id AND wof_geometries.source = excluded.source 
"#;

pub const GET_SRID: &'static str = r#"
SELECT Find_SRID('public', 'wof_geometries', 'geometry') as srid;
"#;
