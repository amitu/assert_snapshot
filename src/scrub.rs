use std::collections::HashMap;

pub fn scrub(content: &str, scrubs: &HashMap<String, String>) -> String {
    let mut content = content.to_owned();
    for (key, val) in scrubs.iter() {
        content = content.replace(key, val);
    }
    content
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn scrub() {
        let mut s = HashMap::new();
        s.insert("world".into(), "foo".into());

        assert_eq!(
            super::scrub("hello world. goodbye world.", &s),
            "hello foo. goodbye foo.",
        );
    }
}
