use crate::{
    models::{artist::Artist, Name},
    utils::error::Error,
};
use sqlx::{PgPool, query_unchecked};
use ulid::Ulid;

impl Artist {

	pub fn new(id: String, name: Name) -> Self {
		Self {
			id: Ulid::from_string(&id).unwrap(),
			name,
		}
	}

	pub async fn get_artists_by_song_id(id: &Ulid, db: &PgPool) -> Result<Vec<Self>, Error> {
		let artist = query_unchecked!(
			r#"
			select
				a.id,
				a.name as "name!: Name"
			from
				songs_artists sa
			left join artists a on (a.id = sa.artist_id)
			where sa.song_id = $1;
			"#, id.to_string()
		)
		.fetch_all(db)
		.await
		.unwrap();

		let mut artists: Vec<Artist> = Vec::new();

		for a in artist {
			let res_artist = Artist::new(a.id, a.name);
			artists.push(res_artist);
		}


		Ok(artists)
	}


}