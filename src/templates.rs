use std::fmt::Write;

use serde::{Deserialize, Serialize};

pub fn output_templates(templates: &Templates, writer: &mut impl Write) -> eyre::Result<()> {
    let templates_with_unicode = serde_json::to_string_pretty(templates)?;
    for c in templates_with_unicode.chars() {
        if c as u32 <= 127 {
            write!(writer, "{c}")?;
        } else {
            write!(writer, r"\u{:4x}", c as u32)?;
        }
    }
    Ok(())
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Templates {
    templates: Vec<Template>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Template {
    name: String,
    filename: String,
    icon_code: String,
    categories: Vec<String>,
}

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
                icon_code: "".to_string(),
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
                icon_code: "".to_string(),
                categories: vec!["Life/organize".to_string()],
            }],
        };
        let mut output = String::new();
        output_templates(&actual, &mut output).expect("Should have written output");

        insta::assert_snapshot!(output);
    }

    #[test]
    fn it_should_round_trip() {
        let actual: Templates = serde_json::from_str(include_str!("../data/example_template.json"))
            .expect("Failed to deserialize");
        let mut output = String::new();
        output_templates(&actual, &mut output).expect("Should have written output");

        insta::assert_snapshot!(output);
    }
}
