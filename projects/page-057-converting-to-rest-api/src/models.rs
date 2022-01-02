use super::schema::cats;

#[derive(diesel::Queryable, serde::Serialize)]
pub struct Cat {
    pub id: i32,
    pub name: String,
    pub image_path: String,
}

#[derive(diesel::Insertable, serde::Serialize, serde::Deserialize)]
#[table_name = "cats"]
pub struct NewCat {
    // id will be added to the database
    pub name: String,
    pub image_path: String,
}
