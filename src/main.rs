mod routes;
mod markdown;

#[macro_use] extern crate rocket;

use std::{io, thread};
use rocket::{Build, Rocket};
use crate::markdown::file_channel::{articles, update_pool};
use crate::routes::articles::get_articles;


#[launch]
fn rocket() -> Rocket<Build> {
    let s = articles();
    listen_to_commands();

     rocket::build()
        .register("/", catchers![routes::catchers::not_found_catcher])
        .mount("/blog", routes![routes::articles::test_article_get, routes::articles::get_articles, routes::articles::get_article])
}

fn listen_to_commands() {
    let handle = thread::spawn( ||{
        loop {
            let mut command = String::new();

            io::stdin()
                .read_line(&mut command)
                .expect("Failed to read command");

            if command.trim() == "--update" {
                update_pool();
            }
        }
    });

}