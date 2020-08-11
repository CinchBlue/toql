use toql_core::sql_mapper_registry::SqlMapperRegistry;
use toql_core::query_parser::QueryParser;
use toql_core::sql_builder::SqlBuilder;

use toql_core::sql_mapper::SqlMapper;
use toql_core::alias_translator::AliasTranslator;
use toql_core::alias::AliasFormat;
use toql_core::sql_expr_parser::SqlExprParser;

struct User {}
struct Book {}

fn setup_registry() -> SqlMapperRegistry {

    let mut registry = SqlMapperRegistry::new();

    let mut mapper = SqlMapper::new::<User>("User");
    mapper
        .map_column("id", "id")
        .map_column("username", "username")
        .map_join("book", "Book", SqlExprParser::parse("INNER JOIN Book ...").unwrap(),  SqlExprParser::parse("..booki_id = ...id").unwrap());
    registry.insert(mapper);

    let mut mapper = SqlMapper::new::<Book>("Book");
    mapper
        .map_column("id", "id")
        .map_column("isbn", "isbn")
        .map_column("publishedAt", "published_at");
  
    registry.insert(mapper);

        registry
}

#[test]
fn delete_simple() {
    let registry = setup_registry();
    let query = QueryParser::parse::<User>("id eq 1").unwrap();
    let sql = SqlBuilder::new("User", &registry)
        .build_delete_sql(&query, "", "", AliasFormat::Canonical).unwrap();

    assert_eq!("DELETE user FROM User user WHERE user.id = 1", sql.unsafe_sql());
}

#[test]
fn delete_joined() {
    let registry = setup_registry();
    let query = QueryParser::parse::<User>("book_isbn eq 1").unwrap();
    let sql = SqlBuilder::new("User", &registry)
        .build_delete_sql(&query, "", "", AliasFormat::Canonical).unwrap();

    assert_eq!("DELETE user FROM User user JOIN Book book WHERE book.isbn = 1", sql.unsafe_sql());
}



