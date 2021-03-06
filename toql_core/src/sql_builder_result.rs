//!
//! Result of SQL Builder. Use it to get SQL that can be sent to the database.

use crate::query::Concatenation;


/// The SQL Builder Result is created by the [SQL Builder](../sql_builder/struct.SqlBuilder.html).
pub struct SqlBuilderResult {
     
    pub(crate) table: String,
    pub(crate) any_selected: bool,
     pub(crate) distinct: bool,
    pub(crate) join_clause: String,
    pub(crate) select_clause: String,
    pub(crate) where_clause: String,
    pub(crate) order_by_clause: String,
    pub(crate) having_clause: String,
    pub(crate) where_params: Vec<String>,
    pub(crate) having_params: Vec<String>,
    pub(crate) combined_params: Vec<String>,
   
}

impl SqlBuilderResult {

    /// Returns true if no field is neither selected nor filtered.
    pub fn is_empty (&self) -> bool{
            !self.any_selected
        &&  self.where_clause.is_empty()
        &&  self.having_clause.is_empty()
    }
    
    fn sql_body(&self,  s: &mut String)  {
        if self.distinct {
            s.push_str(" DISTINCT ");
        }
        s.push_str(&self.select_clause);
        s.push_str(" FROM ");
        s.push_str(&self.table);
         if !self.join_clause.is_empty() {
            s.push(' ');
            s.push_str(&self.join_clause);
         }
         if !self.where_clause.is_empty() {
            s.push_str(" WHERE " );
            s.push_str(&self.where_clause);
         }
         if !self.having_clause.is_empty() {
           s.push_str(" HAVING " );
            s.push_str(&self.having_clause);
         }
         if !self.order_by_clause.is_empty() {
            s.push_str(" ORDER BY " );
            s.push_str(&self.order_by_clause);
         }
         
    }
    
    /// Returns SQL for MySql. 
    /// This function is only available, if you have the _mysql_ feature enabled in your Toql dependency.
    #[cfg(feature = "mysqldb")]
    pub fn to_sql_for_mysql(&self, hint:&str, offset:u64, max: u16) -> String {

        let mut s = String::from("SELECT ");

        if !hint.is_empty() {
            s.push_str(hint);
              s.push(' ');
        }
      
         self.sql_body(&mut s);
         s.push_str(" LIMIT ");
         s.push_str(&offset.to_string());
         s.push(',');
         s.push_str(&max.to_string());

         s
    }
    /// Returns simple SQL.
    pub fn to_sql(&self) -> String {

          let mut s = String::from("SELECT ");
           self.sql_body( &mut s);
           s
    }
    /// Returns SQL parameters for the WHERE and HAVING clauses in SQL.
    pub fn params(&self) -> &Vec<String> {
        if self.where_params.is_empty() {
            &self.having_params
        } else if self.having_params.is_empty() {
            &self.where_params
        } else {
            &self.combined_params
        }
    }

    pub (crate) fn push_pending_parens(clause: &mut String, pending_parens: &u8) {
        for _n in 0..*pending_parens {
            clause.push_str("(");
        }
    }
    pub (crate) fn push_concatenation(clause: &mut String, pending_concatenation: &Option<Concatenation>) {
        if let Some(c) = pending_concatenation {
            match c {
                Concatenation::And => clause.push_str(" AND "),
                Concatenation::Or => clause.push_str(" OR "),
            }
        }
    }
    pub (crate) fn push_filter(clause: &mut String, filter: &str) {
        clause.push_str(filter);
    }
}