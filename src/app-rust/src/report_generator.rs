use handlebars::Handlebars;
use serde_json::Value;

pub fn generate_report(
    template_text: &str,
    origin_data: &Value
) -> Result<String, handlebars::RenderError> {
    let mut handlebars = Handlebars::new();

    handlebars.register_template_string("t1", template_text)?;

    handlebars.render("t1", origin_data)
}
