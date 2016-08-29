use iron::Request;

use models::user::User;

mod authorizer;
mod same_user_auth;
mod logged_in;

pub trait UserRequirement {
    fn check(&self, user: Option<&User>, req: &mut Request) -> bool;
}

pub use self::authorizer::Authorizer;
pub use self::same_user_auth::SameUserAuth;
pub use self::logged_in::LoggedIn;