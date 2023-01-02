

mod article {
    use crate::markdown::article::Article;
    use crate::markdown::attributes::Attributes;


    #[test]
    fn test_from_string() {
        let input = "
title: Wow;
desc: test;
date: 2.01.2023;
--|-
## This is some random article";
        let body = String::from("\n## This is some random article");
        let attributes = Attributes {
            title: String::from("Wow"),
            description: String::from("test"),
            date: String::from("2.01.2023")
        };

        let wanted_result = Article {
            attributes,
            body,
            id: 1
        };
        assert_eq!(wanted_result, Article::from_string(String::from(input), 1))
    }

    #[test]
    fn test_separate_attributes() {
        fn test_from_string() {
            let input = "
title: Wow;
desc: test;
date: 2.01.2023;
--|-
## This is some random article";

            let wanted_result = (String::from("## This is some random article"), String::from("
title: Wow;
desc: test;
date: 2.01.2023;"));

            assert_eq!(wanted_result, Article::separate_attributes(&String::from(input)))
        }
    }}