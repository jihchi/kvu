pub const FLAG_CREATE: &str = "create";
pub const FLAG_UPDATE: &str = "update";
pub const FLAG_DELETE: &str = "delete";

pub enum Operation<'a> {
    Create(&'a str),
    Update(&'a str),
    Upsert(&'a str),
    Delete,
}
