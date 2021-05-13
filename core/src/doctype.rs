use std::usize;

pub trait DocumentationType {
    fn get_root_path() -> String;
    fn get_type_name() -> String;
    fn get_type_name_plural() -> String;
    fn get_type_icon() -> String;

    fn get_path(&self) -> String;
    fn get_link(&self) -> String;

    fn get_type_link() -> String {
        format!("[{} {}]({})",
            Self::get_type_icon(),
            Self::get_type_name_plural(),
            Self::get_root_path()
        )
    }

    fn get_type_name_for_qty(i: &usize) -> String {
        if *i == 1 {
            Self::get_type_name()
        }
        else {
            Self::get_type_name_plural()
        }
    }
}
