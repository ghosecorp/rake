use std::collections::HashMap;

pub trait TemplateEngine: Send + Sync + 'static {
    fn render(&self, template: &str, context: &HashMap<String, String>) -> String;
    fn render_str(&self, template: &str, context: &HashMap<String, String>) -> String;
}