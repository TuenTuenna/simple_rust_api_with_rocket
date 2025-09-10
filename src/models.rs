use serde::Serialize;


#[derive(Serialize)]
pub struct User {
  pub id: String,
  pub name: String, 
  pub age: u32
}

#[derive(Serialize)]
pub struct Post {
  pub id: String,
  pub title: String, 
  pub body: String
}

#[derive(Serialize)]
pub struct Response<T> where T: Serialize {
  pub data: Vec<T>,
  pub message: String,
}