use std::fmt::Write;

use serde::{Deserialize, Serialize};

pub fn output_templates(templates: &Templates, writer: &mut impl Write) -> eyre::Result<()> {
    tracing::info!("Writing templates");
    let templates_with_unicode = serde_json::to_string_pretty(templates)?;
    let mut buf = [0, 0];
    for c in templates_with_unicode.chars() {
        if c.is_ascii() {
            write!(writer, "{c}")?;
        } else {
            tracing::trace!("Writing C-style Unicode escape for {}", c);
            let buf = c.encode_utf16(&mut buf);
            for i in buf {
                write!(writer, r"\u{:4x}", i)?;
            }
        }
    }
    tracing::info!("Wrote templates");
    Ok(())
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Templates {
    templates: Vec<Template>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
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

    #[test]
    fn it_should_reject_new_top_level_keys() {
        let actual = serde_json::from_str::<Templates>(include_str!(
            "../data/invalid_template_new_top_level_key.json"
        ))
        .expect_err("Should have failed to parse");
        assert!(dbg!(actual.to_string()).contains("unknown field `extra`"))
    }

    #[test]
    fn it_should_reject_new_template_level_keys() {
        let actual = serde_json::from_str::<Templates>(include_str!(
            "../data/invalid_template_new_template_level_key.json"
        ))
        .expect_err("Should have failed to parse");
        assert!(dbg!(actual.to_string()).contains("unknown field `extra`"))
    }
}
