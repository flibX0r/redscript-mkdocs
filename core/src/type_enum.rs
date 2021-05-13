use askama::Template;

use crate::doctype::DocumentationType;

#[derive(Debug, Clone, Copy)]
pub struct EnumField<'a> {
    pub name: &'a str,
    pub value: &'a str,
} 

#[derive(Debug, Clone, Copy, Template)]
#[template(path="enum-full.md")]
pub struct Enum<'a> {
    pub name: &'a str,
    pub fields: &'a Vec<EnumField<'a>>,
} 

impl<'a> DocumentationType for Enum<'_> {
    fn get_root_path() -> String {
        String::from("/enum")
    }
    fn get_type_name() -> String {
        String::from("Enum")
    }
    fn get_type_name_plural() -> String {
        String::from("Enums")
    }
    fn get_type_icon() -> String {
        String::from(":material-format-list-bulleted:")
    }
    
    fn get_path(&self) -> String {
        format!("{}/{}",
            Enum::get_root_path(),
            self.name
        )
    }
    fn get_link(&self) -> String {
        format!("[{}]({})",
            self.name,
            self.get_path()
        )
    }
}
#[derive(Debug, Clone, Copy, Template)]
#[template(path="enum-group.md")]
pub struct EnumGroup<'a> {
    pub name: &'a str,
    pub enums: &'a Vec<Enum<'a>>,
}

impl<'a> DocumentationType for EnumGroup<'_> {
    fn get_root_path() -> String {
        String::from("/enum/group")
    }
    fn get_type_name() -> String {
        String::from("Enum Group")
    }
    fn get_type_name_plural() -> String {
        String::from("Enum Groups")
    }
    fn get_type_icon() -> String {
        String::from(":material-view-list-outline:")
    }

    fn get_path(&self) -> String {
        format!("{}/{}",
            Enum::get_root_path(),
            self.name
        )
    }
    fn get_link(&self) -> String {
        format!("[{}]({:?})",
            self.name,
            self.get_path()
        )
    }
}
