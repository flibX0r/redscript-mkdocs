use askama::Template;

pub struct SourceFile<'a> {
    pub filename: &'a str,
    pub fullpath: &'a str,

}
