use datafusion::prelude::SessionContext;
use deltalake::{
    operations::{create::CreateBuilder, write::WriteBuilder},
    storage::DeltaObjectStore,
    Schema,
};
use log::info;
use std::{collections::HashMap, env, path::Path, sync::Arc};
use tempfile::TempDir;
use url::Url;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), deltalake::DeltaTableError> {
    env_logger::init();
    let ctx = SessionContext::new();
    let state = ctx.state();

    // Create the external table pointing to a public parquet file
    ctx.sql(
        format!(
            "CREATE EXTERNAL TABLE supply_chains STORED AS PARQUET LOCATION '{}'",
            env::args().nth(1).expect("No file for profiling provided")
        )
        .as_str(),
    )
    .await?;

    let scan = ctx
        .table_provider("supply_chains")
        .await?
        .scan(&state, None, &[], None)
        .await?;

    let tmp_dir = TempDir::new()?;
    let canonical_path = Path::new(&tmp_dir.path().display().to_string()).canonicalize()?;
    let url = Url::from_directory_path(canonical_path).unwrap();
    let object_store = Arc::from(DeltaObjectStore::try_new(url, HashMap::default())?);
    let delta_schema = Schema::try_from(scan.schema())?;

    info!("Creating the empty table");
    let table = CreateBuilder::default()
        .with_object_store(object_store)
        .with_columns(delta_schema.get_fields().clone())
        .await?;

    info!("Starting the write");
    let table = WriteBuilder::new(table.object_store(), table.state)
        .with_input_execution_plan(scan)
        .with_input_session_state(state)
        .await?;

    info!("Created table {table}");

    Ok(())
}
