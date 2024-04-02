mod routes;
mod handlers;
mod models;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let connection_string = "Server=localhost,1433;Database=rust_blog;User Id=sa;Password=P45Sw0rd!;Encrypt=false;TrustServerCertificate=true;";
    let pool = sqlx::mssql::MssqlPoolOptions::new()
        .connect(connection_string)
        .await?;

    // Example query
    let row: (i32,) = sqlx::query_as("SELECT 1")
        .fetch_one(&pool)
        .await?;

    println!("Query result: {}", row.0);

    Ok(())
}
