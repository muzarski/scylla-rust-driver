use assert_matches::assert_matches;
use scylla::client::session::Session;

use crate::utils::{create_new_session_builder, setup_tracing, unique_keyspace_name, PerformDDL};
use scylla::errors::{BadQuery, ExecutionError};
use scylla::response::query_result::QueryResult;
use scylla::statement::batch::Batch;
use scylla::statement::batch::BatchType;
use scylla::statement::unprepared::Statement;

#[tokio::test]
async fn test_large_batch_statements() {
    setup_tracing();
    let mut session = create_new_session_builder().build().await.unwrap();

    let ks = unique_keyspace_name();
    session = create_test_session(session, &ks).await;

    let max_queries = u16::MAX as usize;
    let batch_insert_result = write_batch(&session, max_queries, &ks).await;

    batch_insert_result.unwrap();

    let too_many_queries = u16::MAX as usize + 1;
    let batch_insert_result = write_batch(&session, too_many_queries, &ks).await;
    assert_matches!(
        batch_insert_result.unwrap_err(),
        ExecutionError::BadQuery(BadQuery::TooManyQueriesInBatchStatement(_too_many_queries)) if _too_many_queries == too_many_queries
    )
}

async fn create_test_session(session: Session, ks: &String) -> Session {
    session
        .ddl(
            format!("CREATE KEYSPACE {} WITH REPLICATION = {{ 'class' : 'NetworkTopologyStrategy', 'replication_factor' : 1 }}",ks),
        )
        .await.unwrap();
    session
        .ddl(format!(
            "CREATE TABLE {}.pairs (dummy int, k blob, v blob, primary key (dummy, k))",
            ks
        ))
        .await
        .unwrap();
    session
}

async fn write_batch(
    session: &Session,
    n: usize,
    ks: &String,
) -> Result<QueryResult, ExecutionError> {
    let mut batch_query = Batch::new(BatchType::Unlogged);
    let mut batch_values = Vec::new();
    let statement_str = format!("INSERT INTO {}.pairs (dummy, k, v) VALUES (0, ?, ?)", ks);
    let statement = Statement::new(statement_str);
    let prepared_statement = session.prepare(statement).await.unwrap();
    for i in 0..n {
        let mut key = vec![0];
        key.extend(i.to_be_bytes().as_slice());
        let value = key.clone();
        let values = vec![key, value];
        batch_values.push(values);
        batch_query.append_statement(prepared_statement.clone());
    }
    session.batch(&batch_query, batch_values).await
}
