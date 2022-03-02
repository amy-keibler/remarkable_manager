use serde::{Deserialize, Serialize, Serializer};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Templates {
    templates: Vec<Template>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Template {
    name: String,
    filename: String,
    icon_code: IconCode,
    categories: Vec<String>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct IconCode(String);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_should_deserialize_a_valid_template() {
        let actual: Templates = serde_json::from_str(include_str!("../data/example_template.json"))
            .expect("Failed to deserialize");

        let expected = Templates {
            templates: vec![Template {
                name: "Burndown".to_string(),
                filename: "burndown".to_string(),
                icon_code: IconCode("\u{e9fe}".to_string()),
                categories: vec!["Life/organize".to_string()],
            }],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn it_should_serialize_a_valid_template() {
        let actual = Templates {
            templates: vec![Template {
                name: "Burndown".to_string(),
                filename: "burndown".to_string(),
                icon_code: IconCode("\u{e9fe}".to_string()),
                categories: vec!["Life/organize".to_string()],
            }],
        };

        insta::assert_json_snapshot!(actual);
    }

    #[test]
    fn it_should_round_trip() {
        let actual: Templates = serde_json::from_str(include_str!("../data/example_template.json"))
            .expect("Failed to deserialize");

        insta::assert_json_snapshot!(actual);
    }
}
