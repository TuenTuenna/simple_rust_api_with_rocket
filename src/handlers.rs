
use crate::models::{ User, Post, Response };
use rocket::serde::json::Json;

#[get("/")]
pub fn index() -> &'static str {
    "오늘도 빡코딩!!"
}

#[get("/ppakcoding")]
pub fn ppakcoding() -> &'static str {
    "빡코딩빡코딩빡코딩!!"
}

#[get("/posts")]
pub fn get_posts() -> Json<Response<Post>> {

  let posts = vec![
    Post { id: "1".to_string(), title: "첫글입니다".to_string(), body: "호롤롤로".to_string() },
    Post { id: "2".to_string(), title: "첫글입니다1".to_string(), body: "호롤롤로1".to_string() },
    Post { id: "3".to_string(), title: "첫글입니다2".to_string(), body: "호롤롤로2".to_string() },
  ];

    Json(Response{
      data: posts,
      message: "성공".to_string()
    })
}

#[get("/users")]
pub fn get_users() -> Json<Response<User>> {
    let users = vec![
    User { id: "1".to_string(), name: "유저1".to_string(), age: 100 },
    User { id: "2".to_string(), name: "유저2".to_string(), age: 101 },
    User { id: "3".to_string(), name: "유저3".to_string(), age: 102 },
  ];

    Json(Response{
      data: users,
      message: "성공".to_string()
    })
}

#[get("/users/<id>")]
pub fn get_user(id: u32) -> String {
    format!("유저 ID: {} -> 하하하 ", id)
}