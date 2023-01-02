#[cfg(test)]
mod attributes {
    use rocket::serde::de::Unexpected::Str;
    use crate::markdown::attributes::Attributes;

    #[test]
    fn test_get_title_attribute() {

        let input = "
        title: Wow;
        desc: test;
        date: 2.01.2023;";
        assert_eq!(String::from("Wow"), Attributes::get_attribute(&String::from(input), "title:"))
    }


    #[test]
    fn test_get_no_title_attribute() {
        let input = "
        desc: test;
        date: 2.01.2023;";

        assert_eq!(String::from(""), Attributes::get_attribute(&String::from(input), "title:"))
    }

    #[test]
    fn test_get_description_attribute() {
        let input = "
        title: wow;
        desc: test;
        date: 2.01.2023;";

        assert_eq!(String::from("test"), Attributes::get_attribute(&String::from(input), "desc:"))
    }


    #[test]
    fn test_get_date_attribute() {

        let input = "
        title: Wow;
        desc: test;
        date: 2.01.2023;";


        assert_eq!(String::from("2.01.2023"), Attributes::get_attribute(&String::from(input), "date:"))
    }

    #[test]
    fn test_parse_from_string() {
        let input = "
        title: Wow;
        desc: test;
        date: 2.01.2023;";


        let wanted_result = Attributes {
            title: String::from("Wow"),
            description: String::from("test"),
            date: String::from("2.01.2023")
        };

        assert_eq!(wanted_result, Attributes::parse_from_string(&String::from(input)))

    }
}
