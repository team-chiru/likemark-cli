#[derive(Serialize, Deserialize)]
pub struct Likemark {
  id: u32,
  parentId: u32,
  title: u32,
  url: u32,
  children: Vec<Likemark>
}