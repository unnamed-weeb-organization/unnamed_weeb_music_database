/* Enum 'ext_sites' */
CREATE TYPE ext_sites AS ENUM('AppleMusic','Youtube','Spotify');

/* Enum 'releasetype' */
CREATE TYPE releasetype AS ENUM('Album','Single','EP');

/* Composite 'localized_name' */
CREATE TYPE localized_name AS (native text, romanized text, english text);

/* Table 'songs' */
DROP TABLE IF EXISTS songs;
CREATE TABLE songs(
	id text NOT NULL,
	"name" localized_name,
	external_sites ext_sites,
	PRIMARY KEY(id)
);

/* Table 'artists' */
DROP TABLE IF EXISTS artists;
CREATE TABLE artists(
	id text NOT NULL,
	"name" localized_name,
	external_sites ext_sites,
	PRIMARY KEY(id)
);

/* Table 'releases' */
DROP TABLE IF EXISTS releases;
CREATE TABLE releases(
	id text NOT NULL,
	"name" localized_name,
	release_type releasetype,
	total_tracks integer,
	PRIMARY KEY(id)
);

/* Table 'songs_artists' */
DROP TABLE IF EXISTS songs_artists;
CREATE TABLE songs_artists(
	id integer NOT NULL,
	artist_id text NOT NULL,
	song_id text NOT NULL,
	PRIMARY KEY(id)
);

/* Table 'songs_releases' */
DROP TABLE IF EXISTS songs_releases;
CREATE TABLE songs_releases(
	id integer NOT NULL,
	songs_id text NOT NULL,
	releases_id text NOT NULL,
	PRIMARY KEY(id)
);

/* Table 'songs_external_sites' */
DROP TABLE IF EXISTS s;
CREATE TABLE songs_external_sites(
	id integer NOT NULL,
	songs_id text NOT NULL,
	"type" ext_sites,
	url text,
	PRIMARY KEY(id)
);

/* Relation 'artists_releases' */
ALTER TABLE releases
	ADD CONSTRAINT artists_releases FOREIGN KEY (artists) REFERENCES artists (id);

/* Relation 'artists_songs_artists' */
ALTER TABLE songs_artists
	ADD CONSTRAINT artists_songs_artists
    	FOREIGN KEY (artist_id) REFERENCES artists (id);

/* Relation 'songs_songs_artists' */
ALTER TABLE songs_artists
	ADD CONSTRAINT songs_songs_artists FOREIGN KEY (song_id) REFERENCES songs (id)
;

/* Relation 'songs_songs_releases' */
ALTER TABLE songs_releases
	ADD CONSTRAINT songs_songs_releases FOREIGN KEY (songs_id) REFERENCES songs (id)
;

/* Relation 'releases_songs_releases' */
ALTER TABLE songs_releases
	ADD CONSTRAINT releases_songs_releases
    	FOREIGN KEY (releases_id) REFERENCES releases (id);

/* Relation 'songs_songs_external_sites' */
ALTER TABLE songs_external_sites
	ADD CONSTRAINT songs_songs_external_sites
    	FOREIGN KEY (songs_id) REFERENCES songs (id);
