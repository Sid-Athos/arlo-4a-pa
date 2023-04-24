use std::ptr::null;
use bcrypt::{hash, verify};
use axum::http::StatusCode;
use rand::distributions::Alphanumeric;
use rand::{Rng, thread_rng};
use crate::database::database_error::{database_error_invalid_input, DatabaseError};