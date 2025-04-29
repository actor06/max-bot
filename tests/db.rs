use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use sqlx::Row;

#[tokio::test]
async fn test_db_connection() -> Result<(), sqlx::Error> {
    dotenv().ok();
    let pool = PgPoolOptions::new()
    .max_connections(1)
    .connect(&std::env::var("DATABASE_URL").unwrap())
    .await?;

    let row = sqlx::query("SELECT 42 as test_value")
    .fetch_one(&pool)
    .await?;

    let value: i32 = row.get("test_value");
    assert_eq!(value, 42);
    Ok(())
}

#[tokio::test]
async fn test_db_tables_exist() -> Result<(), sqlx::Error> {
    dotenv().ok();
    let pool = PgPoolOptions::new()
    .connect(&std::env::var("DATABASE_URL").unwrap())
    .await?;

    let tables = sqlx::query(
        "SELECT table_name FROM information_schema.tables WHERE table_schema = 'public'"
    )
    .fetch_all(&pool)
    .await?;

    let mut table_names = Vec::new();
    for table in tables {
        let name: String = table.get("table_name");
        table_names.push(name);
    }

    assert!(table_names.contains(&"users".to_string()));
    assert!(table_names.contains(&"cycles".to_string()));
    Ok(())
}
