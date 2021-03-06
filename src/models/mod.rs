pub mod user;
// pub mod session;
pub mod schema;
pub mod user_role;
pub mod user_profile;
pub mod image;
pub mod submission;
pub mod invite;
pub mod filter_settings;
pub mod unique_code;

pub trait HasOwner {
    fn get_owner(id: i64) -> Result<Option<self::user::User>, ::error::FurryError>;
}
