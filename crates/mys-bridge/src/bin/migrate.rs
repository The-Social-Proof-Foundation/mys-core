use std::env;
use std::error::Error as StdError;
use std::path::Path;
use std::process;

fn main() {
    // Use tokio runtime directly instead of macro to avoid ABI issues
    let rt = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime");
    rt.block_on(async_main()).unwrap_or_else(|e| {
        eprintln!("❌ Migration failed: {}", e);
        process::exit(1);
    });
}

async fn async_main() -> Result<(), Box<dyn StdError>> {
    println!("🚀 MySo Bridge Database Migration Runner");

    // Get database URL from environment
    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL environment variable must be set");

    println!("🔗 Connecting to database...");

    // Connect to database
    let (mut client, connection) = tokio_postgres::connect(&database_url, tokio_postgres::NoTls)
        .await
        .map_err(|e| format!("Failed to connect to database: {}", e))?;

    // Spawn connection task
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Database connection error: {}", e);
        }
    });

    println!("✅ Connected to database successfully");

    // Create migrations table if it doesn't exist
    create_migrations_table(&mut client).await?;

    // Run migrations
    run_migrations(&mut client).await?;

    println!("🎉 All migrations completed successfully!");
    Ok(())
}

async fn create_migrations_table(
    client: &mut tokio_postgres::Client,
) -> Result<(), Box<dyn StdError>> {
    client
        .execute(
            "CREATE TABLE IF NOT EXISTS _migrations (
            id SERIAL PRIMARY KEY,
            migration_name VARCHAR(255) NOT NULL UNIQUE,
            applied_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
        )",
            &[],
        )
        .await?;

    println!("📊 Migration tracking table ready");
    Ok(())
}

async fn run_migrations(client: &mut tokio_postgres::Client) -> Result<(), Box<dyn StdError>> {
    let migrations_dir = "migrations";

    if !Path::new(migrations_dir).exists() {
        println!("⚠️  No migrations directory found, skipping migrations");
        return Ok(());
    }

    // Read migration files
    let mut migration_files = std::fs::read_dir(migrations_dir)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension()? == "sql" {
                Some(path.file_name()?.to_string_lossy().to_string())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    migration_files.sort();

    for migration_file in migration_files {
        // Check if migration was already applied
        let count: i64 = client
            .query_one(
                "SELECT COUNT(*) FROM _migrations WHERE migration_name = $1",
                &[&migration_file],
            )
            .await?
            .get(0);

        if count > 0 {
            println!("⏭️  Skipping already applied migration: {}", migration_file);
            continue;
        }

        println!("🔄 Applying migration: {}", migration_file);

        // Read migration file
        let migration_path = format!("{}/{}", migrations_dir, migration_file);
        let migration_sql = std::fs::read_to_string(&migration_path)?;

        // Execute migration in a transaction
        let tx = client.transaction().await?;

        // Execute the migration SQL
        tx.batch_execute(&migration_sql).await?;

        // Record migration as applied
        tx.execute(
            "INSERT INTO _migrations (migration_name) VALUES ($1)",
            &[&migration_file],
        )
        .await?;

        tx.commit().await?;

        println!("✅ Successfully applied migration: {}", migration_file);
    }

    Ok(())
}
