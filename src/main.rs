use std::{fs::{self, File}, collections::HashMap};
use std::io::Write;

use template_engine::{get_content_type, ContentType, generate_html_template_var};

fn main() -> Result<(), std::io::Error> {
    let input_file_name = "index.html";
    let output_file_name = "index.rhtml";
    let contents = fs::read_to_string(input_file_name).unwrap();

    // Sample context
    let mut context = HashMap::new();
    context.insert("name".to_string(), "Bob".to_string());
    context.insert("city".to_string(), "Oskemen".to_string());

    let mut output = File::create(output_file_name).unwrap();
    
    for line in contents.lines() {
        match get_content_type(line) {
            ContentType::Literal(text) => {
                let s = text.as_str();
                write!(output, "{}\n", s)?;
            },
            ContentType::TemplateVariable(expr) => {
                write!(output, "{}\n", generate_html_template_var(expr, &context))?;
            },
            ContentType::Tag(_) => {
                write!(output, "{}\n", line.to_string())?;
            },
            ContentType::Unrecognized => {
                panic!("Unrecognized line");
            }
        }
    }

    Ok(())
}
