#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::collections::HashMap;

fn as_fuddy(content: &mut String, reg: &Regex, replace: &str) {
    let result = reg.replace_all(content, replace).to_string();
    content.clear();
    content.push_str(&result);
}

fn get_expressions() -> HashMap<&'static str, &'static str> {
    [
        (r#"(r|l)"#, "w"),
        (r#"qu"#, "qw"),
        (r#"th(\s)"#, "f$1"),
        (r"th", "d"),
        (r#"n\."#, "n, uh-hah-ha-ha."),
        (r"(R|L)", "W"),
        (r"(Qu|QW)", "QW"),
        (r"TH(\s)", "F$1"),
        (r"Th", "D"),
        (r"N\.", "N, uh-hah-hah-hah"),
    ]
    .iter()
    .cloned()
    .collect()
}

pub fn get_fudd(input: &str) -> String {
    let mut result = input.to_string();
    lazy_static! {
        static ref EXPRESSIONS: Vec<(&'static str, &'static str)> = get_expressions()
            .into_iter()
            .map(|(k, v)| (k, v))
            .collect::<Vec<(&str, &str)>>();
        static ref REGEXS: Vec<Regex> = EXPRESSIONS
            .iter()
            .map(|(k, _)| (Regex::new(k).unwrap()))
            .collect::<Vec<Regex>>();
    };

    for (i, (_, v)) in EXPRESSIONS.iter().enumerate() {
        as_fuddy(&mut result, &REGEXS[i], v)
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::get_fudd;

    #[test]
    fn given_rabbits_then_expect_wabbits() {
        let result = get_fudd("rabbits");
        assert_eq!(result, "wabbits");
    }

    #[test]
    fn given_quiet_then_expect_qwiet() {
        let result = get_fudd("quiet");
        assert_eq!(result, "qwiet");
    }

    #[test]
    fn given_thats_then_expect_dats() {
        let result = get_fudd("thats");
        assert_eq!(result, "dats");
    }
}
