//!
//! The query parser can turn a string that follows the Toql query syntax into a [Query](../query/struct.Query.html).
//!
//! ## Example
//!
//! ``` ignore
//! let  query = QueryParser::parse("*, +username").unwrap();
//! assert_eq!("*, +username", query.to_string());
//! ```
//! Read the guide for more information on the query syntax.
//!
//! The parser is written with [Pest](https://pest.rs/) and is fast. It should be used to parse query request from users.
//! To build a query within your program, build it programmatically with the provided methods.
//! This avoids typing mistakes and - unlike parsing - cannot fail.
//!
use crate::error::ToqlError;
use crate::query::Concatenation;
use crate::query::Field;
use crate::query::Predicate;
use crate::query::FieldFilter;
use crate::query::FieldOrder;
use crate::query::Query;
use crate::query::QueryToken;
use crate::query::Wildcard;
use pest::error::Error;
use pest::error::ErrorVariant::CustomError;
use pest::Parser;

#[derive(Parser)]
#[grammar = "toql.pest"]
struct PestQueryParser;

/// The query parser.
/// It contains only a single static method to turn a string into a Query struct.
pub struct QueryParser;

impl QueryParser {
    /// Method to parse a string
    /// This fails if the syntax is wrong. The original PEST error is wrapped with the ToqlError and
    /// can be used to examine to problem in detail.
    pub fn parse(toql_string: &str) -> Result<Query, ToqlError> {
        let pairs = PestQueryParser::parse(Rule::query, toql_string)?;

        let mut query = Query::new();
        let mut con = Concatenation::And;


        fn unquote(quoted: &str) -> String{
            if quoted.starts_with("'") {
                quoted.trim_matches('\'').replace("''", "'")
            } else {
                quoted.to_string()
            }
        }

        for pair in pairs.flatten().into_iter() {
            let span = pair.clone().as_span();
            //   println!("Rule:    {:?}", pair.as_rule());
            //   println!("Span:    {:?}", span);
            //   println!("Text:    {}", span.as_str());
            match pair.as_rule() {
                Rule::field_clause => {
                    query.tokens.push(QueryToken::Field(Field {
                        concatenation: con.clone(),
                        name: "missing".to_string(),
                        hidden: false,
                        order: None,
                        aggregation: false,
                        filter: None,
                    }));
                },
                Rule::sort => {
                    let token = query.tokens.last_mut();
                    if let Some(t) = token {
                        if let QueryToken::Field(ref mut field) = t {
                            let p = span.as_str()[1..].parse::<u8>().unwrap_or(1);
                            if let Some('+') = span.as_str().chars().next() {
                                field.order = Some(FieldOrder::Asc(p));
                            } else {
                                field.order = Some(FieldOrder::Desc(p));
                            }
                        }
                    }
                }
                Rule::hidden => {
                    let token = query.tokens.last_mut();
                    if let Some(t) = token {
                        if let QueryToken::Field(ref mut field) = t {
                            field.hidden = true;
                        }
                    }
                }
                Rule::aggregation => {
                    let token = query.tokens.last_mut();
                    if let Some(t) = token {
                        if let QueryToken::Field(ref mut field) = t {
                            field.aggregation = true;
                        }
                    }
                }

                Rule::field_path => {
                    let token = query.tokens.last_mut();
                    if let Some(t) = token {
                        if let QueryToken::Field(ref mut field) = t {
                            field.name = span.as_str().to_string();
                        }
                    }
                }
                Rule::wildcard_path => {
                    let token = query.tokens.last_mut();
                    if let Some(t) = token {
                        if let QueryToken::Wildcard(ref mut wildcard) = t {
                            wildcard.path = span.as_str().to_string();
                        }
                    }
                }
                Rule::field_filter => {
                    let token = query.tokens.last_mut();
                    if let Some(t) = token {
                        if let QueryToken::Field(ref mut field) = t {
                            let mut iter = span.as_str().split_whitespace();
                            let op = iter.next();

                            field.filter = match op.unwrap().to_uppercase().as_str() {
                                "EQ" => Some(FieldFilter::Eq(unquote(iter.next().unwrap()))),
                                "EQN" => Some(FieldFilter::Eqn),
                                "NE" => Some(FieldFilter::Ne(unquote(iter.next().unwrap()))),
                                "NEN" => Some(FieldFilter::Nen),
                                "GT" => Some(FieldFilter::Gt(unquote(iter.next().unwrap()))),
                                "GE" => Some(FieldFilter::Ge(unquote(iter.next().unwrap()))),
                                "LT" => Some(FieldFilter::Lt(unquote(iter.next().unwrap()))),
                                "LE" => Some(FieldFilter::Le(unquote(iter.next().unwrap()))),
                                "LK" => Some(FieldFilter::Lk(unquote(iter.next().unwrap()))),
                                "IN" => Some(FieldFilter::In(iter.map(unquote).collect())),
                                "OUT" => Some(FieldFilter::Out(iter.map(unquote).collect())),
                                "BW" => Some(FieldFilter::Bw(
                                    unquote(iter.next().unwrap()),
                                    unquote(iter.next().unwrap()),
                                )),
                                "RE" => Some(FieldFilter::Re(unquote(iter.next().unwrap()))),
                                //     "SC" => Some(FieldFilter::Sc(iter.next().unwrap().to_string())),
                                "FN" => Some(FieldFilter::Fn(
                                    unquote(iter.next().unwrap()),
                                    iter.map(unquote).collect(),
                                )),
                                _ => {
                                    return Err(ToqlError::QueryParserError(Error::new_from_span(
                                        CustomError {
                                            message: "Invalid filter Function".to_string(),
                                        },
                                        span,
                                    )))
                                }
                            }
                        }
                    }
                },
                 Rule::predicate_clause => {
                    query.tokens.push(QueryToken::Predicate(Predicate {
                        concatenation: con.clone(),
                        name: "missing".to_string(),
                        args: Vec::new(),
                    }));
                },
                 Rule::predicate_name =>  {
                    let token = query.tokens.last_mut();
                    if let Some(t) = token {
                        if let QueryToken::Predicate(ref mut predicate) = t {
                              predicate.name = span.as_str().trim_start_matches("@").to_string();
                        }
                    }
                },
                Rule::predicate_arg =>  {
                    let token = query.tokens.last_mut();
                    if let Some(t) = token {
                        if let QueryToken::Predicate(ref mut predicate) = t {
                              let iter = span.as_str().split_whitespace();
                                predicate.args = iter.map(unquote).collect();
                        }
                    }
                },
                Rule::wildcard => {
                    query.tokens.push(QueryToken::Wildcard(Wildcard {
                        concatenation: con.clone(),
                        path: String::from(""),
                    }));
                }
                Rule::rpar => {
                    query.tokens.push(QueryToken::RightBracket);
                }
                Rule::lpar => {
                    query.tokens.push(QueryToken::LeftBracket(con.clone()));
                }
                Rule::concat => {
                    if let Some(',') = span.as_str().chars().next() {
                        con = Concatenation::And;
                    } else {
                        con = Concatenation::Or;
                    }
                }

                _ => {}
            }
        }
        Ok(query)
    }
}
