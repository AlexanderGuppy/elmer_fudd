#[macro_use]
extern crate lazy_static;
use regex::Regex;
use regex::RegexSet;
use std::collections::HashMap;

fn as_fuddy(content: &mut String, reg: &str, replace: &str) {
    let reg = Regex::new(reg).unwrap();
    let result = reg.replace_all(content, replace).to_string();
    content.clear();
    content.push_str(&result);
}

fn as_fuddy2(content: &mut String, reg: &str, replace: &str) {
    let reg = Regex::new(reg).unwrap();
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
        static ref HASHMAP: HashMap<&'static str, &'static str> = get_expressions();
        static ref EXPRESSIONS: Vec<&'static str> = get_expressions()
            .iter()
            .map(|(k, _)| k)
            .collect::<Vec<&'static str>>();
        static ref SET: RegexSet = RegexSet::new(EXPRESSIONS.iter()).unwrap();
    };

    //std::thread::sleep(std::time::Duration::from_nanos(10));
    //    let hm = get_expressions();
    for (k, v) in HASHMAP.iter() {
        //HASHMAP.iter() {
        as_fuddy(&mut result, k, v);
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
}
