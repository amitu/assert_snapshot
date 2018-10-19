use itertools::Itertools;
use regex::Regex;
use std::{fs::File, io::Read};

fn till_line(content: &str, line: u32) -> String {
    let lines: Vec<String> = content.split("\n").map(|s| s.to_string()).collect();
    let (length, _) = lines.iter().size_hint();
    lines.iter().rev().skip(length - (line as usize)).join("\n")
}

fn from_content(content: &str, line: u32) -> String {
    let content = till_line(content, line);
    let re = Regex::new(
        r#"(?x)
        \bfn[[:space:]]*([[:alnum:]]+)[[:space:]]*[\(<]
    "#,
    ).unwrap();
    re.captures_iter(&content).next().unwrap()[1].to_string()
}

pub fn function(file: &'static str, line: u32) -> String {
    let mut file = File::open(&file).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    from_content(&content, line)
}

#[cfg(test)]
mod tests {
    fn t(c: &str, l: u32, f: &str) {
        assert_eq!(super::from_content(c, l), f);
    }

    #[test]
    fn from_content() {
        t("fn foo(){}", 1, "foo");
        t("let baz = 10;\nfn foo(){}\nfn bar(){}", 2, "foo");
        t("fn foo(){}\nfn bar(){}", 2, "bar");
        t("fn foo<asd>(){}", 1, "foo");
        t("fn foo<asd>(){}\nfn bar(){}", 1, "foo");
        t("fn foo<asd>(){}\nfn bar(){}", 2, "bar");
    }

    #[test]
    fn function() {
        assert_eq!(super::function(file!(), line!()), "function")
    }
}
