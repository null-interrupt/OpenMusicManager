use super::external_urls::ExternalURLs;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct TrackLink {
	pub external_urls: ExternalURLs,
	pub href: String,
	pub id: String,
	#[serde(alias = "type")]
	pub link_type: String,
	pub hate: String,
}