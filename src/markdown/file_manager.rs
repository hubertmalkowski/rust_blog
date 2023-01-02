use std::{fs};
use crate::markdown::article::Article;


pub fn get_articles_vector() -> Vec<String> {
    let mut result = Vec::new();

    if let Ok(entries) = fs::read_dir("articles") {
        for entry in entries {

            if let Ok(entry) = entry {
                if let Ok(file_type) = entry.file_type() {
                    println!("{:?}: {:?}", entry.file_name(),file_type );
                    let file = fs::read_to_string(entry.path())
                        .expect("Failed to open a file");
                    result.push(file);
                }
            }
        }
    }
     result
}

pub fn get_articles() -> Vec<Article> {
    let articles_string = get_articles_vector();
    let mut result : Vec<Article> = Vec::new();
    articles_string.iter().enumerate().for_each(|(i, el)|{
        result.push(Article::from_string(el.clone(), i as u32))
    });
    result
}