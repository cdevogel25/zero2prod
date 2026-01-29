use actix_session::{Session, SessionExt, SessionGetError, SessionInsertError};
use actix_web::{FromRequest, HttpRequest, dev::Payload};
use std::future::{Ready, ready};

use uuid::Uuid;

pub struct TypedSession(Session);

impl TypedSession {
    const USER_ID_KEY: &'static str = "user_id";

    pub fn renew(&self) {
        self.0.renew();
    }

    pub fn insert_user_id(&self, user_id: Uuid) -> Result<(), SessionInsertError> {
        self.0.insert(Self::USER_ID_KEY, user_id)
    }

    pub fn get_user_id(&self) -> Result<Option<Uuid>, SessionGetError> {
        self.0.get(Self::USER_ID_KEY)
    }

    pub fn log_out(self) {
        self.0.purge();
    }
}

impl FromRequest for TypedSession {
    // here we are saying "we return the same error returned
    // by the implementation of `FromRequest` for `Session`".
    type Error = <Session as FromRequest>::Error;

    // dangit! I thought I had one!!
    // but 1.75 does support `async` in traits so what gives?
    // is it an actix-session issue?
    type Future = Ready<Result<TypedSession, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        ready(Ok(TypedSession(req.get_session())))
    }
}
