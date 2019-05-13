#Toql derive reference

The derive provides struct level attributes and field level attributes.

## Attributes for structs

Attribute | Description                             | Example / Remark
tables  |   Defines for struct and join how table names are generated. Possible values are SHOUTY_SNAKE_CASE, .. | tables="SHOUTY_SNAKE_CASE" on struct User will select from table USER
columns        | Same as Attribute tables for columns. | columns="mixedCase" 
table | Sets the table name for a struct or join. | table ="User" on struct `NewUser` will access table `User`
skip_query | Derive does not generate query functionality for the struct. | No load_one<User>(...)
skip_query_builder | Derive does not generate field functions |  No User::fields.id() 
skip_indelup | Derive does not generate insert / delete / update functionlity. | No insert_one<User>(...)

## Attributes for fields  


Attribute | Description | Example / Remark
delup_key | Field used as key for delete and update functions. For composite keys, use multiple times. |
skip_inup | No insert / update for this field. | Use for auto increment columns or columns calculated from database triggers.
sql       | Map field to SQL expression instead of tabl column. To assign the table alias use two dots. Skipped for insert / update | sql ="SELECT COUNT (*) FROM Message m WHERE m.user_id = ..id"
sql_join  | Loads a single struct with an sql join, where self and other defines columns with same values. For composite keys unse multiple joins. Option<> will cause a left join. If se         |sql_join( self="friend_id", other="friend.id", on="friend.best = true") If _self_ is omitted it will be created from Variable name. 
merge     | Loads a dependend Vec<>. Merges run an additional SELECT statemen. self and other define struct fields with same values. For composite fields use multiple merges | merge(self="id", other="user_id")
ignore_wildcard | No selection for ** and *
alias | Build sql_join with this alias | XX
table | Join or merge on this table | XX
role | Field only accessable for querries with certain roles. Multiple use requires multiple roles. | role="admin" role= "superadmin" 