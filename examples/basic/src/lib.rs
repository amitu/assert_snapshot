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
        assert_snapshot!(&json!({}), &HashMap::new());
        assert_snapshot!("tastic", &json!({}), &HashMap::new());
    }
}
