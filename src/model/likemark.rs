#[derive(Serialize, Deserialize, Debug)]
pub struct Likemark {
  id: String,
  #[serde(default, rename = "parentId")] parent_id: String,
  #[serde(default)] title: String,
  #[serde(default)] url: String,
  #[serde(default)] children: Vec<Likemark>
}