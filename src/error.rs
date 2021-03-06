#![allow(dead_code)]

use iron::prelude::*;
use iron::status;
use bcrypt::BcryptError;

use diesel;
use std::error::Error;
use models::user::User;
use std::fmt;

quick_error! {
    #[derive(Debug)]
    pub enum FurryError {
        NotImplemented(err: String) {
            description(err)
        }
        Template(err: Box<Error + Send>) {
            cause(&**err)
            description(err.description())
            from(e: fmt::Error) -> (Box::new(e))
        }
        Database(err: Box<Error + Send>) {
            cause(&**err)
            description(err.description())
            from(e: diesel::result::Error) -> (Box::new(e))
        }
        Login(err: Box<Error + Send>) {
            cause(&**err)
            description(err.description())
            //from(e: FurryError) -> (Box::new(e))
        }
        Bcrypt(err: BcryptError) {
            from()
        }
        Internal(err: Box<Error + Send>) {
            cause(&**err)
            description(err.description())
        }
        Image(err: ::image::ImageError) {
            from()
        }
        Io(err: ::std::io::Error) {
            from()
        }
        Parse(err: Option<Box<Error + Send>>) {
            from(e: ::std::num::ParseIntError) -> (Some(Box::new(e)))
            from(e: ::rustc_serialize::base64::FromBase64Error) -> (Some(Box::new(e)))
        }
        Unauthorized(user: Option<User>) {}
        NotFound {}
        BadFormatting {}
        MailError(err: Box<Error + Send>) {
            cause(&**err)
            description(err.description())
            from(e: ::lettre::transport::smtp::error::Error) -> (Box::new(e))
        }
    }
}

impl From<FurryError> for IronError {
    fn from(e: FurryError) -> IronError {
        IronError::new(Box::new(e), status::InternalServerError)
    }
}
