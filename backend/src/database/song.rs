use crate::{
    models::{song::Song, Name, artist, release, ExternalSites},
    utils::error::Error,
};
use hyper::StatusCode;
use sqlx::{PgPool, Row};
use ulid::Ulid;

impl Song {
	pub fn new(name: Name, artists: Vec<artist::Artist>, releases: Vec<release::Release>, external_sites: Vec<ExternalSites>) -> Self {
		Self {
			id: Ulid::new().to_string(),
			name,
		}
	}

	/// Returns a song with the given id from the database.
	///
	/// # Arguments
	/// * `id` - The id of the song to be returned.
	/// # Errors
	/// * `Error::NotFound` - If the song with the given id does not exist.
	/// # Returns
	/// * `Song` - The song with the given id.
	pub async fn get(id: &str, db: &PgPool) -> Result<Self, Error> {
		let row = sqlx::query(
			r#"
			select
				s.*, array_agg(a.*) as artists, array_agg(r.*) as releases
			from
				songs s
			left join songs_artists on (songs_artists.song_id = s.id)
			left join artists a on (a.id = songs_artists.artist_id)
			left join songs_releases on (songs_releases.songs_id = s.id)
			left join releases r on (r.id = songs_releases.releases_id)
			group by s.id;
			"#,
		)
		.bind(id)
		.fetch_one(db)
		.await
		.unwrap();

	println!("{:?}", row.columns());


	let song = Self {
		id: row.get("id"),
		name: row.get("name"),
	};


	Ok(song)
	}

}