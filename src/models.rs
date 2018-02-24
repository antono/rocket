#[derive(Queryable)]
pub struct User {
    pub user_id: i32,
    pub email: String,
    pub published: bool,
}