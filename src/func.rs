use fancy_regex::Regex;
use uuid::Uuid;

pub fn is_naming_ok(name: &Option<String>) -> Option<String> {
    match name.is_some() {
        true => {
            let regx = Regex::new(r"^[a-z|A-Z][a-z|A-Z\-\d]{2,20}$").unwrap();
            let some = name.as_ref().unwrap().clone();
            return match regx.is_match(some.as_str()).unwrap() {
                true => Some(some),
                false => None
            };
        }
        false => None
    }
}

pub fn unique_id() -> String {
    format!("{}", Uuid::new_v4().simple())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name_check() {
        let chk = is_naming_ok(&Some("abc".to_string()));
        assert!(chk.is_some());
    }
}