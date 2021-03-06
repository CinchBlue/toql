//!
//! This module contains the query and all functions to build one programatically.
//!
//! ## Example
//!
//! ```ignore
//! use toql::query::{Query, Field};
//! 
//! let  q = Query::new()
//!        .and(Field::from("foo").hide().eq(5).aggregate().asc(1))
//!        .and(Field::from("bar").desc(2));
//!    assert_eq!("+1.foo !EQ 5,-2bar", q.to_string());
//! ```
//!
//! To avoid typing mistakes the Toql derive builds functions for all fields in a struct.
//! In the example above it would be possible to write
//! `.and(Foobar::fields().bar().desc(2)` for a derived struct `Foobar`.
//!
//! Read the guide for more information on the query syntax.
//!
use std::collections::BTreeSet;
use std::fmt;

/// A trait to convert a simple datatype into a filter argument. Used by builder functions. Not very interesting ;)
pub trait FilterArg<T> {
    fn to_sql(self) -> String;
}

impl FilterArg<&str> for &str {
    fn to_sql(self) -> String {
        let mut s = String::from("'");
        // TODO escape for swl
        s.push_str(self);
        s.push('\'');
        s
    }
}

impl FilterArg<u8> for u8 {
    fn to_sql(self) -> String {
        self.to_string()
    }
}
impl FilterArg<u16> for u16 {
    fn to_sql(self) -> String {
        self.to_string()
    }
}
impl FilterArg<u32> for u32 {
    fn to_sql(self) -> String {
        self.to_string()
    }
}
impl FilterArg<u64> for u64 {
    fn to_sql(self) -> String {
        self.to_string()
    }
}
impl FilterArg<u128> for u128 {
    fn to_sql(self) -> String {
        self.to_string()
    }
}
impl FilterArg<i8> for i8 {
    fn to_sql(self) -> String {
        self.to_string()
    }
}
impl FilterArg<i16> for i16 {
    fn to_sql(self) -> String {
        self.to_string()
    }
}
impl FilterArg<i32> for i32 {
    fn to_sql(self) -> String {
        self.to_string()
    }
}
impl FilterArg<i64> for i64 {
    fn to_sql(self) -> String {
        self.to_string()
    }
}
impl FilterArg<i128> for i128 {
    fn to_sql(self) -> String {
        self.to_string()
    }
}
impl FilterArg<bool> for bool {
    fn to_sql(self) -> String {
        String::from(if self == true { "1" } else { "0" })
    }
}
impl FilterArg<f32> for f32 {
    fn to_sql(self) -> String {
        self.to_string()
    }
}
impl FilterArg<f64> for f64 {
    fn to_sql(self) -> String {
        self.to_string()
    }
}

#[derive(Clone, Debug)]
pub(crate) enum Concatenation {
    And,
    Or,
}

/// A wildcard is used to select all fields from top or from a path.
///
/// Example
/// ```ignore
///
///  let q = Query::new().and(Wildcard::new()).and(Wildcard::from("bar")); // more elegant -> Query::wildcard().and(...)
///
///  assert_eq!("*, bar_*", q.to_string());
/// ```
/// Note that the Toql derive builds a wildcard function too.
/// If a struct `Foo` contained a struct `Bar`, it would be possible to replace the second call to _and()_ with  `.and(Bar::fields().bar().wildcard())`
#[derive(Clone, Debug)]
pub struct Wildcard {
    pub(crate) concatenation: Concatenation,
    pub(crate) path: String,
}

impl Wildcard {
    /// Creates a new wildcard to select all fields from top
    pub fn new() -> Self {
        Wildcard {
            concatenation: Concatenation::And,
            path: String::from(""),
        }
    }
    /// Creates a new wildcard to select all fields from a path
    pub fn from<T>(path: T) -> Self
    where
        T: Into<String>,
    {
        let mut path = path.into();
        #[cfg(debug)]
        {
            if !path.chars().all(|x| x.is_alphanumeric() || x == '_') {
                panic!(
                    "Path {:?} must only contain alphanumeric characters and underscores.",
                    path
                );
            }
        }
        // Remove optional trainling *
        if path.ends_with("*") {
            path.pop();
        }
        // Add _ at end if missing
        if !path.ends_with("_") {
            path.push('_');
        }

        Wildcard {
            concatenation: Concatenation::And,
            path: path.into(),
        }
    }
}

/// A Toql field can select, filter and order a database column or expression
/// A field can be created from a field name and filtered, sorted with its methods.
/// However the Toql derive creates fields structs for a derived struct, so instead of
/// ``` ignore
///  
///  let f = Field::from("id");
/// ```
/// its easier and recommended to write
/// ``` ignore
///  let f = User::fields().id();
/// ```
#[derive(Clone, Debug)]
pub struct Field {
    pub(crate) concatenation: Concatenation,
    pub(crate) name: String,
    pub(crate) hidden: bool,
    pub(crate) order: Option<FieldOrder>,
    pub(crate) filter: Option<FieldFilter>,
    pub(crate) aggregation: bool,
}

impl Field {
    /// Create a field for the given name.
    pub fn from<T>(name: T) -> Self
    where
        T: Into<String>,
    {
        let name = name.into();
        #[cfg(debug)]
        {
            // Ensure name does not end with wildcard
            if name.ends_with("*") {
                panic!("Fieldname {:?} must not end with wildcard.", name);
            }
        }

        Field {
            concatenation: Concatenation::And,
            name: name.into(),
            hidden: false,
            order: None,
            filter: None,
            aggregation: false,
        }
    }
    /// Hide field. Useful if a field should not be selected, but be used for filtering.
    pub fn hide(mut self) -> Self {
        self.hidden = true;
        self
    }
    /// Aggregate a field to make the filter be in SQL HAVING clause instead of WHERE clause
    pub fn aggregate(mut self) -> Self {
        self.aggregation = true;
        self
    }
    /// Use this field to order records in ascending way. Give ordering priority when records are ordered by multiple fields.
    pub fn asc(mut self, order: u8) -> Self {
        self.order = Some(FieldOrder::Asc(order));
        self
    }
    /// Use this field to order records in descending way. Give ordering priority when records are ordered by multiple fields.
    pub fn desc(mut self, order: u8) -> Self {
        self.order = Some(FieldOrder::Desc(order));
        self
    }
    /// Filter records with _equal_ predicate.
    pub fn eq<T>(mut self, criteria: impl FilterArg<T>) -> Self {
        self.filter = Some(FieldFilter::Eq(criteria.to_sql()));
        self
    }
    /// Filter records with _equal null_ predicate.
    pub fn eqn(mut self) -> Self {
        self.filter = Some(FieldFilter::Eqn);
        self
    }
    /// Filter records with _not equal_ predicate.
    pub fn ne<T>(mut self, criteria: impl FilterArg<T>) -> Self {
        self.filter = Some(FieldFilter::Ne(criteria.to_sql()));
        self
    }
    /// Filter records with _not equal null_ predicate.
    pub fn nen(mut self) -> Self {
        self.filter = Some(FieldFilter::Nen);
        self
    }
    /// Filter records with greater that_ predicate.
    pub fn gt<T>(mut self, criteria: impl FilterArg<T>) -> Self {
        self.filter = Some(FieldFilter::Gt(criteria.to_sql()));
        self
    }
    /// Filter records with greater or equal_ predicate.
    pub fn ge<T>(mut self, criteria: impl FilterArg<T>) -> Self {
        self.filter = Some(FieldFilter::Ge(criteria.to_sql()));
        self
    }
    /// Filter records with lesser than_ predicate.
    pub fn lt<T>(mut self, criteria: impl FilterArg<T>) -> Self {
        self.filter = Some(FieldFilter::Lt(criteria.to_sql()));
        self
    }
    /// Filter records with lesser or equal_ predicate.
    pub fn le<T>(mut self, criteria: impl FilterArg<T>) -> Self {
        self.filter = Some(FieldFilter::Le(criteria.to_sql()));
        self
    }
    /// Filter records with _between_ predicate. This is inclusive, so `x bw 3 6` is the same as `x ge 3, x le 6`
    pub fn bw<T>(mut self, lower: impl FilterArg<T>, upper: impl FilterArg<T>) -> Self {
        self.filter = Some(FieldFilter::Bw(lower.to_sql(), upper.to_sql()));
        self
    }
    /// Filter records with _like_ predicate.
    pub fn lk<T>(mut self, criteria: impl FilterArg<T>) -> Self {
        self.filter = Some(FieldFilter::Lk(criteria.to_sql()));
        self
    }
    /// Filter records with _regex_ predicate.
    pub fn re<T>(mut self, criteria: impl FilterArg<T>) -> Self {
        self.filter = Some(FieldFilter::Re(criteria.to_sql()));
        self
    }
    /*   pub fn sc<T>(mut self, criteria: impl FilterArg<T>) -> Self {
        self.filter = Some(FieldFilter::Sc(criteria.to_sql()));
        self
    } */
    /// Filter records with _inside_ predicate.
    pub fn ins<T>(mut self, criteria: Vec<impl FilterArg<T>>) -> Self {
        self.filter = Some(FieldFilter::In(
            criteria.into_iter().map(|c| c.to_sql()).collect(),
        ));
        self
    }
    /// Filter records with _outside_ predicate.
    pub fn out<T>(mut self, criteria: Vec<impl FilterArg<T>>) -> Self {
        self.filter = Some(FieldFilter::Out(
            criteria.into_iter().map(|c| c.to_sql()).collect(),
        ));
        self
    }
    /// Filter records with custom function.
    /// To provide a custom function you must implement (FieldHandler)[../sql_mapper/trait.FieldHandler.html]
    /// See _custom handler test_ for an example.
    pub fn fnc<U, T>(mut self, name: U, args: Vec<impl FilterArg<T>>) -> Self
    where
        U: Into<String>,
    {
        self.filter = Some(FieldFilter::Fn(
            name.into(),
            args.into_iter().map(|c| c.to_sql()).collect(),
        ));
        self
    }
}

impl ToString for Field {
    fn to_string(&self) -> String {
        let mut s = String::new();
        match self.order {
            Some(FieldOrder::Asc(o)) => {
                s.push('+');
                s.push_str(&o.to_string());
            }
            Some(FieldOrder::Desc(o)) => {
                s.push('-');
                s.push_str(&o.to_string());
            }
            None => {}
        };
        if self.hidden {
            s.push('.');
        }
        s.push_str(&self.name);

        if self.filter.is_some() {
            if self.aggregation {
                s.push_str(" !");
            } else {
                s.push(' ');
            }
        }
        match self.filter {
            None => s.push_str(""),
            Some(FieldFilter::Eq(ref arg)) => {
                s.push_str("EQ ");
                s.push_str(arg);
            }
            Some(FieldFilter::Eqn) => {
                s.push_str("EQN");
            }
            Some(FieldFilter::Ne(ref arg)) => {
                s.push_str("NE ");
                s.push_str(arg);
            }
            Some(FieldFilter::Nen) => {
                s.push_str("NEN");
            }
            Some(FieldFilter::Gt(ref arg)) => {
                s.push_str("GT ");
                s.push_str(arg);
            }
            Some(FieldFilter::Ge(ref arg)) => {
                s.push_str("GE ");
                s.push_str(arg);
            }
            Some(FieldFilter::Lt(ref arg)) => {
                s.push_str("LT ");
                s.push_str(arg);
            }
            Some(FieldFilter::Le(ref arg)) => {
                s.push_str("LE ");
                s.push_str(arg);
            }
            Some(FieldFilter::Lk(ref arg)) => {
                s.push_str("LK ");
                s.push_str(arg);
            }
            Some(FieldFilter::Re(ref arg)) => {
                s.push_str("RE ");
                s.push_str(arg);
            }
            /*  Some(FieldFilter::Sc(ref arg)) => {
                s.push_str("SC ");
                s.push_str(arg);
            } */
            Some(FieldFilter::Bw(ref lower, ref upper)) => {
                s.push_str("BW ");
                s.push_str(lower);
                s.push(' ');
                s.push_str(upper);
            }
            Some(FieldFilter::In(ref args)) => {
                s.push_str("IN ");
                s.push_str(&args.join(" "))
            }
            Some(FieldFilter::Out(ref args)) => {
                s.push_str("OUT ");
                s.push_str(&args.join(" "))
            }
            Some(FieldFilter::Fn(ref name, ref args)) => {
                s.push_str("FN ");
                s.push_str(name);
                s.push(' ');
                s.push_str(&args.join(" "))
            }
        }
        s
    }
}

impl From<&str> for Field {
    fn from(s: &str) -> Field {
        Field::from(s)
    }
}

impl Into<QueryToken> for Field {
    fn into(self) -> QueryToken {
        QueryToken::Field(self)
    }
}

/// The filter operation on a field. You use this when creating a [FieldHandler](../sql_mapper/trait.FieldHandler.html)
/// to provide custom functions through the _Fn_ filter or implement a alternative mapping to SQL.
#[derive(Clone, Debug)]
pub enum FieldFilter {
    Eq(String),
    Eqn,
    Ne(String),
    Nen,
    Gt(String),
    Ge(String),
    Lt(String),
    Le(String),
    Lk(String),
    Bw(String, String), // Lower, upper limit
    In(Vec<String>),
    Out(Vec<String>),
    Re(String),
    //  Sc(String),
    Fn(String, Vec<String>), // Function name, args
}
#[derive(Clone, Debug)]
pub(crate) enum FieldOrder {
    Asc(u8),
    Desc(u8),
}

#[derive(Clone, Debug)]
pub(crate) enum QueryToken {
    LeftBracket(Concatenation),
    RightBracket,
    Wildcard(Wildcard),
    DoubleWildcard(Concatenation),
    Field(Field),
}

impl From<&str> for QueryToken {
    fn from(s: &str) -> QueryToken {
        if s == "**" {
            QueryToken::DoubleWildcard(Concatenation::And)
        } else if s.ends_with("*") {
            QueryToken::Wildcard(Wildcard::from(s))
        } else {
            QueryToken::Field(Field::from(s))
        }
    }
}

impl ToString for QueryToken {
    fn to_string(&self) -> String {
        let s = match self {
            QueryToken::RightBracket => String::from(")"),
            QueryToken::LeftBracket(c) => match c {
                Concatenation::And => String::from("("),
                Concatenation::Or => String::from("("),
            },
            QueryToken::Field(
                field, /*Field {concatenation, name, hidden, order, filter, aggregation}*/
            ) => field.to_string(),
            QueryToken::Wildcard(wildcard) => format!("{}*", wildcard.path),
            QueryToken::DoubleWildcard(_) => String::from("**"),
        };
        s
    }
}

/// A Query contains fields and wildcards.
/// It can be turned into SQL using the [SQL Builder](../sql_builder/struct.SqlBuilder.html).
///
/// To build a big query simple add fields and wildcards with _and_ resp. _or_ function.
///
/// Watch out: Logical AND has precendence over OR. So `a OR b AND c` is the same as `a OR (b AND c)`.
/// Avoid to restrict your query with a permission field WITHOUT parenthesize the incoming query. 
/// Malicious users will circumvent your permission filter with a simple OR clause at the beginning.
/// Consider this: `(*, id nen); id, permission ne ""` vs `((*, id nen); id), permission ne ""`.
/// To parenthesize a query use the [parenthesize()](struct.Query.html#method.parenthesize) method.
///
/// ``` ignore
/// let q1 = Query::new().and(Field("b").eq(3)).and(Field("c").eq(2));
/// let q2 = Query::new().and(Field("a").eq(1)).or(q1.parens());
///
/// assert_eq!("a eq 1; (b eq 3, c eq 2)", q2.to_string())
#[derive(Clone, Debug)]
pub struct Query {
    pub(crate) tokens: Vec<QueryToken>,
    /// Select DISTINCT
    pub distinct: bool,
    /// Roles a query has to access fields.
    /// See [MapperOption](../sql_mapper/struct.MapperOptions.html#method.restrict_roles) for explanation.
    pub roles: BTreeSet<String>,
}

impl Query {
    /// Create a new empty query.
    pub fn new() -> Self {
        Query {
            tokens: vec![],
            distinct: false,
            roles: BTreeSet::new(),
        }
    }
    /// Create a new query that select all top fields.
    pub fn wildcard() -> Self {
        Query {
            tokens: vec![QueryToken::Wildcard(Wildcard::new())],
            distinct: false,
            roles: BTreeSet::new(),
        }
    }
    /// Create a new query that select all top fields and all dependend fields. This is the best :)
    pub fn double_wildcard() -> Self {
        Query {
            tokens: vec![QueryToken::DoubleWildcard(Concatenation::And)],
            distinct: false,
            roles: BTreeSet::new(),
        }
    }
    /// Wrap query with parentheses.
    pub fn parenthesize(mut self) -> Self  {
        if self.tokens.is_empty() {
            return self;
        }
        self.tokens.insert(0, QueryToken::LeftBracket(Concatenation::And));   
        self.tokens.push(QueryToken::RightBracket);
        self
    }
    /// Concatenate field or query with AND.
    pub fn and<T>(mut self, query: T) -> Self
    where
        T: Into<Query>,
    {
        // All tokens are by default concatenated with AND
        self.tokens.append(&mut query.into().tokens);
        self
    }
    /// Concatenate field or query with OR.
    pub fn or<T>(mut self, query: T) -> Self
    where
        T: Into<Query>,
    {
        // Change first token of query to concatenate with OR
        let mut query = query.into();
        if let QueryToken::LeftBracket(c) = query.tokens.get_mut(0).unwrap() {
            *c = Concatenation::Or;
        } else if let QueryToken::Field(field) = query.tokens.get_mut(0).unwrap() {
            field.concatenation = Concatenation::Or;
        } else if let QueryToken::Wildcard(wildcard) = query.tokens.get_mut(0).unwrap() {
            wildcard.concatenation = Concatenation::Or;
        } else if let QueryToken::DoubleWildcard(w) = query.tokens.get_mut(0).unwrap() {
            *w = Concatenation::Or;
        }
        
        self.tokens.append(&mut query.tokens);
        self
    }
    // Not sure if needed
    /* pub fn prepend<T>(mut self, query: T) -> Self
    where
        T: Into<Query>,
    {
        // Swap queries for better append performance
        let mut q = query.into();
        q.tokens.append(&mut self.tokens);
        std::mem::swap(&mut q.tokens, &mut self.tokens);

        self
    } */
}

// Doc: Display  implements automatically .to_string()
impl fmt::Display for Query {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn get_concatenation(c: &Concatenation) -> char {
            match c {
                Concatenation::And => ',',
                Concatenation::Or => ';',
            }
        }

        let mut s = String::new();
        let mut concatenation_needed = false;

        for token in &self.tokens {
            if concatenation_needed {
                match &token {
                    QueryToken::LeftBracket(concatenation)
                    | QueryToken::DoubleWildcard(concatenation) => {
                        s.push(get_concatenation(concatenation))
                    }
                    QueryToken::Wildcard(wildcard) => {
                        s.push(get_concatenation(&wildcard.concatenation))
                    }
                    QueryToken::Field(field) => s.push(get_concatenation(&field.concatenation)),
                    _ => {}
                }
            }
            s.push_str(&token.to_string());
            match token {
                QueryToken::LeftBracket(..) => concatenation_needed = false,
                QueryToken::Field(..) => concatenation_needed = true,
                QueryToken::Wildcard(..) => concatenation_needed = true,
                QueryToken::DoubleWildcard(..) => concatenation_needed = true,
                _ => {}
            }
        }

        // To avoid allocation check first if string starts with a separator
        let t = s.chars().next().unwrap_or(' ');
        if t == ',' || t == ';' {
            s = s.trim_start_matches(",").trim_start_matches(";").to_owned();
        }

        write!(f, "{}", s)
    }
}

impl From<Field> for Query {
    fn from(field: Field) -> Query {
        let mut q = Query::new();
        q.tokens.push(QueryToken::Field(field));
        q
    }
}

impl From<Wildcard> for Query {
    fn from(wildcard: Wildcard) -> Query {
        let mut q = Query::new();
        q.tokens.push(QueryToken::Wildcard(wildcard));
        q
    }
}

impl From<&str> for Query {
    fn from(string: &str) -> Query {
        let mut q = Query::new();

        q.tokens.push(if string == "**" {
            QueryToken::DoubleWildcard(Concatenation::And)
        } else if string.ends_with("*") {
            QueryToken::Wildcard(Wildcard::from(string))
        } else {
            QueryToken::Field(Field::from(string))
        });
        q
    }
}
