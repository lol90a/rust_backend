//! # app-core
//!
//! Domain logic, application services, infrastructure interfaces, and shared
//! utilities. This crate is deliberately free of HTTP types and can be used
//! independently of the Actix Web server.
//!
//! ## Layer layout
//!
//! ```text
//! domain          – entities, value objects, domain errors, domain services
//! application     – use cases, commands/queries, DTOs, repository ports
//! infrastructure  – concrete adapters (in-memory, DB, cache, queue, …)
//! shared          – cross-cutting types (pagination, request IDs)
//! ```
//!
//! Utility modules (`fs`, `path`, `net`, `string`, `list`, `json`, `parse`,
//! `database`, `workers`, `queue`, `cache`, `broker`) provide helpers that
//! can be used by any layer without violating the dependency rule.

pub mod application;
pub mod broker;
pub mod cache;
pub mod database;
pub mod domain;
pub mod fs;
pub mod infrastructure;
pub mod json;
pub mod list;
pub mod net;
pub mod parse;
pub mod path;
pub mod queue;
pub mod shared;
pub mod string;
pub mod workers;
