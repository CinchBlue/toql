// Toql. Transfer Object Query Language
// Copyright (c) 2019 Roy Ganz
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! # Toql. Transfer Object Query Language
//!
//! Welcome to Toql API documentation!
//! 
//! This API documentation is very technical and is purely a reference. 
//! There is a [guide](https://roy-ganz.github.io/toql/) that is better to get started.
//! 
//! ## Overview 
//! 
//! The project consists of the following main parts:
//!
//!  * A [Query Parser](https://docs.rs/toql_core/0.1/toql_core/query_parser/index.html) to build a Toql query from a string. 
//!  * A [Query](https://docs.rs/toql_core/0.1/toql_core/query/index.html) that can be built with methods.
//!  * A [SQL Mapper](https://docs.rs/toql_core/0.1/toql_core/sql_mapper/index.html) to map Toql fields to database columns or expressions.
//!  * A [SQL Builder](https://docs.rs/toql_core/0.1/toql_core/sql_builder/index.html) to  turn your Toql query into an SQL statement using the mapper.
//!  * A [Toql Derive](https://docs.rs/toql_derive/0.1/index.html) to build all the boilerplate code to make some ✨ happen.
//!  * Integration with
//!      * [MySQL](https://docs.rs/toql_mysql/0.1/index.html)
//!      * [Rocket](https://docs.rs/toql_rocket/0.1/index.html)
//!
//! ## Small Example
//! Using Toql without any dependency features is possible and easy. Here we go:
//! ``` rust
//! use toql::{query_parser::QueryParser, sql_mapper::SqlMapper, sql_builder::SqlBuilder};
//! 
//! let query = QueryParser::parse("id, +title LK '%foo%'").unwrap();
//! let mut mapper = SqlMapper::new("Book b");
//!     mapper
//!         .map_field("id", "b.id")
//!         .map_field("title", "b.title");
//!
//! let result = SqlBuilder::new().build(&mapper, &query).unwrap();
//! assert_eq!("SELECT b.id, b.title FROM Book b WHERE b.title LIKE ? ORDER BY b.title ASC", result.to_sql());
//! ```
//! 
//! ## Bigger Example
//! Have a look at the [CRUD example](https://github.com/roy-ganz/toql/blob/master/examples/rocket_mysql/main.rs) that serves users with Rocket and MySQL.
//! 

pub use toql_core::error;
pub use toql_core::error::Result;
pub use toql_core::query;
pub use toql_core::query_parser;
pub use toql_core::sql_builder;
pub use toql_core::sql_builder_result;
pub use toql_core::sql_mapper;
pub use toql_core::fields_type;
pub use toql_core::merge;
pub use toql_core::indelup;

pub use toql_derive as derive;


pub use log; // Reexport for generated code from Toql derive

#[cfg(feature = "mysql")]
pub use toql_mysql as mysql;
