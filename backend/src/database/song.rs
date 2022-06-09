use crate::{
    models::{song::Song, Name},
    utils::error::Error,
};
use sqlx::{PgPool, query_unchecked};
use ulid::Ulid;

impl Song {
	pub fn new(id: String, name: Name) -> Self {
		Self {
			id: Ulid::from_string(&id).unwrap(),
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
	pub async fn get_song_by_id<S: AsRef<str>> (id: S, db: &PgPool) -> Result<Self, Error> {
		let song = query_unchecked!(
			r#"
			select
				s.id,
				s.name as "name!: Name"
			from
				songs s
			where s.id = $1;
			"#, id.as_ref()
		)
		.fetch_one(db)
		.await
		.unwrap();

		let res_song = Song::new(song.id, song.name);


	Ok(res_song)
	}

}