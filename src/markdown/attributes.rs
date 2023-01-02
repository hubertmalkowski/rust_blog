use std::ops::Deref;
use rocket::serde;
use rocket::serde::{Serialize, Serializer};
use rocket::serde::ser::SerializeStruct;

#[derive(PartialEq)]
#[derive(Debug)]
pub struct Attributes {
    pub(crate) title: String,
    pub(crate) description: String,
    pub(crate) date: String,
}



impl Attributes {
    pub fn parse_from_string(attr : &String) -> Attributes {
        Attributes {
            title: Attributes::get_attribute(&attr, "title:"),
            description: Attributes::get_attribute(&attr, "desc:"),
            date: Attributes::get_attribute(&attr, "date:")
        }
    }

    pub fn get_attribute(attrs: &String, attr : &str) -> String {
        let title : Vec<&str> = attrs
            .lines()
            .filter(|el| {
                el.contains(attr)
            })
            .collect();

        return if title.len() > 0 {
            let mut result = String::from(title[0]);
            result = result.replace(attr, "").replace(";", "");
            result.trim().parse().unwrap()
        } else {
            String::from("")
        }
    }
}

impl Serialize for Attributes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut s = serializer.serialize_struct("Attributes", 3)?;
        s.serialize_field("title", &self.title);
        s.serialize_field("description", &self.description);
        s.serialize_field("date", &self.date);
        s.end()
    }
}