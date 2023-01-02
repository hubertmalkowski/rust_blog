use std::ops::Deref;
use std::sync::mpsc;
use rocket::futures::StreamExt;
use rocket::serde::json::{json, Value};
use crate::markdown::article::Article;
use crate::markdown::file_channel::articles;
use crate::markdown::file_manager::get_articles_vector;


#[get("/test_api")]
pub fn test_article_get() -> Value {

    let article = Article::from_string(get_articles_vector()[0].clone(), 1);

    return json!(article)
}

#[get("/articles")]
pub fn get_articles() -> Value {
    let a = articles();
    let data = a.inner.lock().unwrap();
    return json!(data.deref().clone())
}


#[get("/articles/<i>")]
pub fn get_article(i: u32) -> Value {
    let a = articles();
    let data = a.inner.lock().unwrap();
    let result = data.deref().clone();
    let result : Vec<&Article> = result
        .into_iter()
        .filter(|el| {el.id == i})
        .collect();

    json!(result)
}

