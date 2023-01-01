use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum ContentType {
    Literal(String),
    TemplateVariable(ExpressionData),
    Tag(TagType),
    Unrecognized,
}

#[derive(Debug, PartialEq)]
pub enum TagType {
    ForTag,
    IfTag,
}

#[derive(Debug, PartialEq)]
pub struct ExpressionData {
    pub head: Option<String>,
    pub variable: String,
    pub tail: Option<String>,
}

pub fn get_content_type(line: &str) -> ContentType {
    if check_matching_pair(line, "{{", "}}") {
        ContentType::TemplateVariable(get_expression_data(line))
    } else if check_matching_pair(line, "{%", "%}") {
        ContentType::Tag(get_tag_type(line).unwrap())
    } else {
        ContentType::Literal(line.to_string())
    }
}

fn get_tag_type(input: &str) -> Option<TagType> {
    let for_start = get_index_for_symbol(input, "{%").unwrap() + 2;
    let (_, for_and_tail) = input.split_at(for_start);
    let for_end = get_index_for_symbol(for_and_tail, "%}").unwrap();
    let (for_expr, _) = for_and_tail.split_at(for_end);

    let (expr_type_str, _) = for_expr.trim().split_at(4);
    if expr_type_str.contains("if") { Some(TagType::IfTag) }
    else if expr_type_str.contains("for") { Some(TagType::ForTag) }
    else { None }
}

fn check_matching_pair(input: &str, left_symbol: &str, right_symbol: &str) -> bool {
    input.contains(left_symbol) &&
    input.contains(right_symbol) &&
    get_index_for_symbol(input, left_symbol).unwrap() + 2 < get_index_for_symbol(input, right_symbol).unwrap()
}

fn get_index_for_symbol(input: &str, symbol: &str) -> Option<usize> {
    input.find(symbol)
}

fn get_expression_data(input: &str) -> ExpressionData {
    let var_start = get_index_for_symbol(input, "{{").unwrap() + 2;
    let (head, var_and_tail) = input.split_at(var_start);
    let var_end = get_index_for_symbol(var_and_tail, "}}").unwrap();
    let (var, tail) = var_and_tail.split_at(var_end);

    let head = head.replace("{{", "");
    let tail = tail.replace("}}", "");
    let var = var.trim();

    ExpressionData { head: Some(head.to_string()), variable: var.to_string(), tail: Some(tail.to_string()) }
}

pub fn generate_html_template_var(content: ExpressionData, context: &HashMap<String, String>) -> String {
    let mut html_line = String::new();

    if let Some(h) = content.head {
        html_line.push_str(&h);
    }

    html_line.push_str(context.get(&content.variable).unwrap());

    if let Some(t) = content.tail {
        html_line.push_str(&t);
    }

    html_line    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_literal_test() {
        let input = "<h1>Hello world</h1>";
        assert_eq!(ContentType::Literal(input.to_string()), get_content_type(input));
    }

    #[test]
    fn check_template_var_test() {
        let output = ExpressionData {
            head: Some("Hi ".to_string()),
            variable: "name".to_string(),
            tail: Some(" ,welcome".to_string()),
        };
        assert_eq!(
            ContentType::TemplateVariable(output),
            get_content_type("Hi {{name}} ,welcome"),
            );
    }

    #[test]
    fn check_for_tag_test() {
        assert_eq!(
            ContentType::Tag(TagType::ForTag),
            get_content_type("{% for name in names %}, welcome"),
            );
    }

    #[test]
    fn check_if_tag_test() {
        assert_eq!(
            ContentType::Tag(TagType::IfTag),
            get_content_type("{% if name == 'Bob' %}"),
            );
    }

    #[test]
    fn check_matching_pair_test() {
        assert_eq!(
            true,
            check_matching_pair("{{Hello}}", "{{", "}}"),
            );
    }

    #[test]
    fn check_matching_pair_no_var_test() {
        assert_eq!(
            false,
            check_matching_pair("{{}}", "{{", "}}"),
            );
    }

    #[test]
    fn check_matching_pair_incorrect_pair_test() {
        assert_eq!(
            false,
            check_matching_pair("}}{{", "{{", "}}"),
            );
    }

    #[test]
    fn get_expression_data_test() {
        assert_eq!(
            ExpressionData {
                head: Some("Hi ".to_string()),
                variable: "name".to_string(),
                tail: Some(" ,welcome".to_string()),
            },
            get_expression_data("Hi {{name}} ,welcome"),
            );
    }

    #[test]
    #[should_panic]
    fn get_expression_data_error_test() {
        get_expression_data("Hi {{} ,welcome");
    }

    #[test]
    fn get_tag_type_if_test() {
        assert_eq!(
            TagType::IfTag,
            get_tag_type("{% if name == 'Bob' %}").unwrap(),
            )
    }

    #[test]
    fn get_tag_type_for_test() {
        assert_eq!(
            TagType::ForTag,
            get_tag_type("{% for i in (1..100) %}").unwrap(),
            )
    }

}
