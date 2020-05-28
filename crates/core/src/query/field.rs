
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

use super::concatenation::Concatenation;
use super::field_order::FieldOrder;
use super::field_filter::FieldFilter;
use crate::sql::SqlArg;
use super::QueryToken;
use super::field_path::FieldPath;
use heck::MixedCase;

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
    pub fn canonical_alias(&self, root: &str) -> String {
        format!("{}_{}", root.to_mixed_case(), self.name)

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
    pub fn eq(mut self, criteria: impl Into<SqlArg>) -> Self {
        self.filter = Some(FieldFilter::Eq(criteria.into()));
        self
    }
    /// Filter records with _equal null_ predicate.
    pub fn eqn(mut self) -> Self {
        self.filter = Some(FieldFilter::Eqn);
        self
    }
    /// Filter records with _not equal_ predicate.
    pub fn ne(mut self, criteria: impl Into<SqlArg>) -> Self {
        self.filter = Some(FieldFilter::Ne(criteria.into()));
        self
    }
    /// Filter records with _not equal null_ predicate.
    pub fn nen(mut self) -> Self {
        self.filter = Some(FieldFilter::Nen);
        self
    }
    /// Filter records with greater that_ predicate.
    pub fn gt(mut self, criteria: impl Into<SqlArg>) -> Self {
        self.filter = Some(FieldFilter::Gt(criteria.into()));
        self
    }
    /// Filter records with greater or equal_ predicate.
    pub fn ge(mut self, criteria: impl Into<SqlArg>) -> Self {
        self.filter = Some(FieldFilter::Ge(criteria.into()));
        self
    }
    /// Filter records with lesser than_ predicate.
    pub fn lt(mut self, criteria: impl Into<SqlArg>) -> Self {
        self.filter = Some(FieldFilter::Lt(criteria.into()));
        self
    }
    /// Filter records with lesser or equal_ predicate.
    pub fn le(mut self, criteria: impl Into<SqlArg>) -> Self {
        self.filter = Some(FieldFilter::Le(criteria.into()));
        self
    }
    /// Filter records with _between_ predicate. This is inclusive, so `x bw 3 6` is the same as `x ge 3, x le 6`
    pub fn bw(mut self, lower: impl Into<SqlArg>, upper: impl Into<SqlArg>) -> Self {
        self.filter = Some(FieldFilter::Bw(lower.into(), upper.into()));
        self
    }
    /// Filter records with _like_ predicate.
    pub fn lk(mut self, criteria: impl Into<SqlArg>) -> Self {
        self.filter = Some(FieldFilter::Lk(criteria.into()));
        self
    }
    /// Filter records with _regex_ predicate.
    pub fn re(mut self, criteria: impl Into<SqlArg>) -> Self {
        self.filter = Some(FieldFilter::Re(criteria.into()));
        self
    }
   
    /// Filter records with _inside_ predicate.
    pub fn ins<T, I>(mut self, criteria: I) -> Self
    where T: Into<SqlArg>, I :IntoIterator<Item = T>
     {
        self.filter = Some(FieldFilter::In(
            criteria.into_iter().map(|c| c.into()).collect(),
        ));
        self
    }
    /// Filter records with _outside_ predicate.
    pub fn out<T,I>(mut self, criteria: I) -> Self 
     where T: Into<SqlArg>, I :IntoIterator<Item = T>
    {
        self.filter = Some(FieldFilter::Out(
            criteria.into_iter().map(|c| c.into()).collect(),
        ));
        self
    }
    /// Filter records with custom function.
    /// To provide a custom function you must implement (FieldHandler)[../sql_mapper/trait.FieldHandler.html]
    /// See _custom handler test_ for an example.
    pub fn fnc<U, T, I>(mut self, name: U, args: I) -> Self
    where
        U: Into<String>, T: Into<SqlArg>, I :IntoIterator<Item = T>
    {
        self.filter = Some(FieldFilter::Fn(
            name.into(),
            args.into_iter().map(|c| c.into()).collect(),
        ));
        self
    }

    /// Filter records with custom function.
    /// To provide a custom function you must implement (FieldHandler)[../sql_mapper/trait.FieldHandler.html]
    /// See _custom handler test_ for an example.
    pub fn concatenate(mut self, concatenation: Concatenation) -> Self
    {
        self.concatenation = concatenation;
        self
    }

    pub fn basename(&self) -> &str {
        let i = self.name.rfind('_').unwrap_or(0);
        &self.name[i..]
    }
    pub fn path(&self) -> FieldPath {
        let i = self.name.rfind('_').unwrap_or(0);
        
        FieldPath::from(&self.name[0..i])

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
            None => {}
            Some(ref filter) => {
                s.push_str(filter.to_string().as_str());
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