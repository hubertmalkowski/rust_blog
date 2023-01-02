use rocket::serde;
use rocket::serde::{Serialize, Serializer};
use rocket::serde::ser::SerializeStruct;
use crate::markdown::attributes::Attributes;


#[derive(PartialEq)]
#[derive(Debug)]
pub struct Article {
    pub(crate) attributes: Attributes,
    pub(crate) body: String,
    pub(crate) id : u32
}

impl Article {
    pub fn from_string(article : String, id : u32) -> Article {
        let (body, attrs) = Article::separate_attributes(& article);
        let attributes = Attributes::parse_from_string(&attrs);
        Article {
            attributes,
            body,
            id
        }
    }

    pub fn separate_attributes(article : &String) -> (String, String) {
        let result : Vec<&str> = article.split("--|-").collect();
        (String::from(result[1]), String::from(result[0]))
    }

    pub fn parse_to_markdown() {

    }

}

impl Serialize for Article {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut s = serializer.serialize_struct("Article", 3)?;
        s.serialize_field("attributes", &self.attributes).expect("TODO: panic message");
        s.serialize_field("body", &self.body);
        s.serialize_field("id", &self.id);
        s.end()
    }
}