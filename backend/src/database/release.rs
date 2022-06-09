use crate::{
    models::{Name, release::{Release, ReleaseType}},
    utils::error::Error,
};
use sqlx::{PgPool, query_unchecked};
use ulid::Ulid;

impl Release {

	pub fn new(id: String, name: Name, release_type: ReleaseType, total_tracks: i32) -> Self {
		Self {
			id: Ulid::from_string(&id).unwrap(),
			name,
			release_type,
			total_tracks,
		}
	}

	pub async fn get_releases_by_song_id(id: &Ulid, db: &PgPool) -> Result<Vec<Self>, Error> {
		let release = query_unchecked!(
			r#"
			select
				r.id,
				r.name as "name!: Name",
				r.release_type as "release_type!: ReleaseType",
				r.total_tracks as "total_tracks!: i32"
			from
				songs_releases sr
			left join releases r on (r.id = sr.releases_id)
			where sr.songs_id = $1;
			"#, id.to_string()
		)
		.fetch_all(db)
		.await
		.unwrap();
		
		let mut releases: Vec<Release> = Vec::new();

		for r in release {
			let res_release = Release::new(r.id, r.name, r.release_type, r.total_tracks);
			releases.push(res_release);
		}


		Ok(releases)
	}


}