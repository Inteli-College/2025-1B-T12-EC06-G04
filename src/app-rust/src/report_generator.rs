use std::collections::BTreeMap;
use handlebars::Handlebars;
use serde_json::Value;
use markdown_to_text;

pub fn generate_report(
    template_text: &str,
    origin_data: BTreeMap<String, Value>
) -> Result<String, handlebars::RenderError> {
    let template_text = markdown_to_text::convert(template_text);

    let mut handlebars = Handlebars::new();

    assert!(handlebars.register_template_string("t1", template_text).is_ok());

    let mut data = BTreeMap::new();

    for (l, v) in &origin_data {
        let label = l.to_string().trim().to_string();
        let value = v.to_string().trim().to_string();
        data.insert(label, value);
    }

    return handlebars.render("t1", &data)
}
