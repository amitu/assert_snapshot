pub fn destination(file: &'static str, func: &str, extra: &str, test: bool, ext: &str) -> String {
    let mut dst = file.replace(".rs", "__").to_string();
    dst.push_str(&func);
    if extra != "" {
        dst.push_str("_");
        dst.push_str(extra);
    }
    if test {
        dst.push_str("-test");
    }
    dst.push_str(".");
    dst.push_str(ext);
    dst
}

#[cfg(test)]
mod tests {
    #[test]
    fn destination() {
        assert_eq!(
            super::destination("src/foo.rs", "hello", "yo", false, "json"),
            "src/foo__hello_yo.json"
        );
        assert_eq!(
            super::destination("src/foo.rs", "hello", "", false, "json"),
            "src/foo__hello.json"
        );
        assert_eq!(
            super::destination("src/foo.rs", "hello", "yo", true, "json"),
            "src/foo__hello_yo-test.json"
        );
        assert_eq!(
            super::destination("src/foo.rs", "hello", "", true, "json"),
            "src/foo__hello-test.json"
        );
    }
}
