use anyhow::Error;
use hyper::Method;
use serde::{Deserialize, Deserializer};
use time::OffsetDateTime;

use super::Context;
use crate::{
    http::Request,
    model::{
        channel::*,
        permission::Permissions,
        Id, member::ThreadMember,
        user::User},
};

impl Context {

}
