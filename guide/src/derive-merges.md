
# Merge
A struct can also contain a collection of another struct. Since this cannot be done directly in SQL Toql will execute multiple queries and merge the results afterwards. 

```rust
struct User {
	 id: u32,
	 name: Option<String>
	 #[toql(merge(self="id", other="user_id"))]  // Struct fields for Rust comparison
	 mobile_phones : Vec<Phone>
}

struct Phone {
	number: Option<String>
	user_id : Option<u32>
}
```

Selecting all fields from above with `**` will run 2 SELECT statements and merge the resulting `Vec<Phone>` into `Vec<User>` by the common value of `user.id` and `phone.user_id`.

## Composite fields

To merge on composite fields use the attribute multiple times `#[toql(merge(..), merge(..))`.