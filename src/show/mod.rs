use crate::Secrets;

pub mod gtk4;

pub struct Props {
    pub title: Option<String>,
    pub secrets: Secrets,
}
