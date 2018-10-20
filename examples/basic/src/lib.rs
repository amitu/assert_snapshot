#[cfg(test)]
#[macro_use]
extern crate assert_snapshot;
#[cfg(test)]
#[macro_use]
extern crate serde_json;

#[cfg(test)]
mod tests {
    use assert_snapshot;
    use std::collections::HashMap;

    #[test]
    fn basic() {
        let mut scrubs = HashMap::new();
        scrubs.insert("bamma".into(), "lamma".into());

        assert_snapshot!(&json!({"hello": "world"}), &scrubs);
        assert_snapshot!(&json!({"gamma": "bamma"}), &scrubs; "tastic");
    }

    #[test]
    fn acidic() {
        assert_snapshot!(&json!({"hello": "world"}));
        assert_snapshot!(&json!({"hello": "world"}); "tastic");
    }
}
