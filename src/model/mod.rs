//! This module contains models of the different objects that exist in MISP: Event, Attribute, Object, Organization.
//! It also contains the functions to serialize/deserialize the models to correct JSON objects

pub mod attribute;
pub mod event;
pub mod object;
pub mod organization;
pub mod serialization_helpers;
pub mod server_info;
