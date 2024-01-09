pub mod pass;
pub mod select;
pub mod show;

use std::collections::BTreeMap;
use std::io::{BufRead, Read};

type SecretId = String;

type SecretData = String;

pub struct Secrets(BTreeMap<SecretId, SecretData>);

impl Secrets {
    pub fn new(items: BTreeMap<SecretId, SecretData>) -> Self {
        Self(items)
    }

    pub fn get(&self, id: &str) -> Option<&SecretData> {
        self.0.get(id)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&SecretId, &SecretData)> {
        self.0.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<T> From<T> for Secrets
where
    T: Read + BufRead,
{
    fn from(reader: T) -> Self {
        let mut lines = reader.lines();

        // First line is the password
        let password = lines.next().map(|line| line.unwrap());

        // All other lines are key-value pairs, separated by a colon
        let mut other: BTreeMap<SecretId, SecretData> = lines
            .filter_map(|x| x.ok())
            .take_while(|line| line != "---")
            .filter_map(|line| {
                let mut parts = line.splitn(2, ':');
                let id = parts.next()?;
                let data = parts.next()?.trim_start();
                Some((id.to_string(), data.to_string()))
            })
            .collect();

        // Add the password, only if it is non-empty
        if let Some(password) = password {
            other.insert("password".to_string(), password);
        }

        Self(other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_store_single_line() {
        let input = b"THIS_IS_THE_PASSWORD".as_slice();
        let secrets = Secrets::from(input);
        assert_eq!(secrets.get("password").unwrap(), "THIS_IS_THE_PASSWORD");
    }

    #[test]
    fn test_password_store_multi_line() {
        let input = b"THIS_IS_THE_PASSWORD\nusername: test".as_slice();
        let secrets = Secrets::from(input);
        assert_eq!(secrets.get("password").unwrap(), "THIS_IS_THE_PASSWORD");
        assert_eq!(secrets.get("username").unwrap(), "test");
    }

    #[test]
    fn test_invalid_multiline() {
        let input = b"THIS_IS_THE_PASSWORD\nasdasd\nqweqwe".as_slice();
        let secrets = Secrets::from(input);
        assert_eq!(secrets.get("password").unwrap(), "THIS_IS_THE_PASSWORD");
    }

    #[test]
    fn test_empty_input() {
        let input = b"".as_slice();
        let secrets = Secrets::from(input);
        assert!(secrets.get("password").is_none());
        assert!(secrets.0.is_empty());
    }

    #[test]
    fn test_password_store_multi_line_with_end_delimiter() {
        let input = b"THIS_IS_THE_PASSWORD\n---\nusername: test".as_slice();
        let secrets = Secrets::from(input);
        assert_eq!(secrets.get("password").unwrap(), "THIS_IS_THE_PASSWORD");
        assert!(secrets.get("username").is_none());
    }
}
