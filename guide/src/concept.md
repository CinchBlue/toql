# Concept

Toql is a set of crates that aim to simplify web development:

1. A web client sends a Toql query to the REST Server.
2. The server uses Toql to parse the query and create SQL.
3. SQL is send to the Database.
4. The database results are sent to the client.

While all the low level functions are available for the programmer, the Toql derive produces also high level functions, so that the whole can be done with a single function call.

## Example

Here is some code that uses Rocket to serve users from a database. Note that toql can handle dependencies, such as joins and merges. Cool!

```rust
	#[derive(Toql)]
	#[toql(skip_indelup)] // No insert / delete / update functionality
	struct Country {
		id: String,
		name: Option<String>
	}

	#[derive(Toql)]
	#[toql(skip_indelup)]
	struct User {
		id: u32,
		name: Option<String>,
		#[toql(sql_join(self="country_id", other="id"))]
		country: Option<Country>
	}

    
	#[query("/?<toql..>")]
	fn query(toql: Form<ToqlQuery>,  conn: ExampleDbConnection, 
		mappers: State<SqlMapperCache>) -> Result<Counted<Json<User>>> {
		let ExampleDbConnection(mut c) = conn;

		let r = toql::rocket::load_many(toql, mappers, &mut c)?;
		Ok(Counted(Json(r.0), r.1))
	}

	#[database("example_db")]
	pub struct ExampleDbConnection(mysql::Conn);

	fn main() {
		let mut mappers = SqlMapperCache::new();
		SqlMapper::insert_new_mapper::<User>(&mut mappers);

		rocket::ignite().mount("/query", routes![query]).launch();
	}
```

If you have a MySQL Server running, try the full CRUD example.

```bash
LOGIN=<user>:<pass> cargo run --example rocket_mysql
```

