//! Tests for the db module
//!
//! These tests require an active SSH tunnel to the PostgreSQL database:
//! ssh -L 5433:127.0.0.1:5432 root@144.126.251.126 -N -f
//!
//! Run tests with: cargo test -- --test-threads=1

use super::connection::{create_pool, test_connection, DEFAULT_DATABASE_URL};
use super::memories;
use super::documents;

/// Test that we can create a connection pool successfully
#[tokio::test]
async fn test_create_pool() {
    let pool = create_pool(DEFAULT_DATABASE_URL).await;
    assert!(pool.is_ok(), "Failed to create pool: {:?}", pool.err());
    
    let pool = pool.unwrap();
    let conn_test = test_connection(&pool).await;
    assert!(conn_test.is_ok(), "Connection test failed: {:?}", conn_test.err());
}

/// Test that list_light returns memories
#[tokio::test]
async fn test_list_memories() {
    let pool = create_pool(DEFAULT_DATABASE_URL).await.expect("Pool creation failed");
    
    let notes = memories::list_light(&pool, 10, 0).await;
    assert!(notes.is_ok(), "list_light failed: {:?}", notes.err());
    
    let notes = notes.unwrap();
    assert!(!notes.is_empty(), "Expected memories but got empty list");
    
    // Verify basic field presence
    let first = &notes[0];
    assert!(first.id > 0, "Note should have positive id");
    assert!(!first.path.is_empty(), "Note should have a path");
}

/// Test that get_by_path finds a known memory
#[tokio::test]
async fn test_get_memory_by_path() {
    let pool = create_pool(DEFAULT_DATABASE_URL).await.expect("Pool creation failed");
    
    // First get any note to find a valid path
    let notes = memories::list_light(&pool, 1, 0).await.expect("list_light failed");
    assert!(!notes.is_empty(), "Need at least one memory to test");
    
    let test_path = &notes[0].path;
    
    // Now fetch by that path
    let note = memories::get_by_path(&pool, test_path).await;
    assert!(note.is_ok(), "get_by_path failed: {:?}", note.err());
    
    let note = note.unwrap();
    assert_eq!(&note.path, test_path, "Paths should match");
    assert!(note.id > 0, "Note should have positive id");
}

/// Test that get_count returns a positive count
#[tokio::test]
async fn test_memory_count() {
    let pool = create_pool(DEFAULT_DATABASE_URL).await.expect("Pool creation failed");
    
    let count = memories::get_count(&pool).await;
    assert!(count.is_ok(), "get_count failed: {:?}", count.err());
    
    let count = count.unwrap();
    assert!(count > 0, "Expected positive memory count, got {}", count);
    
    // We know there are ~2,678 memories
    assert!(count > 100, "Expected >100 memories, got {}", count);
}

/// Test search functionality
#[tokio::test]
async fn test_search_memories() {
    let pool = create_pool(DEFAULT_DATABASE_URL).await.expect("Pool creation failed");
    
    // Search for a common term that should exist
    let results = memories::search(&pool, "daily", 10).await;
    assert!(results.is_ok(), "search failed: {:?}", results.err());
    
    let results = results.unwrap();
    // We expect at least some daily notes to exist
    assert!(!results.is_empty(), "Expected search results for 'daily'");
}

/// Test get_by_id functionality
#[tokio::test]
async fn test_get_memory_by_id() {
    let pool = create_pool(DEFAULT_DATABASE_URL).await.expect("Pool creation failed");
    
    // First get any note to find a valid ID
    let notes = memories::list_light(&pool, 1, 0).await.expect("list_light failed");
    assert!(!notes.is_empty(), "Need at least one memory to test");
    
    let test_id = notes[0].id;
    
    // Now fetch by ID
    let note = memories::get_by_id(&pool, test_id).await;
    assert!(note.is_ok(), "get_by_id failed: {:?}", note.err());
    
    let note = note.unwrap();
    assert_eq!(note.id, test_id, "IDs should match");
}

// ============================================================================
// Documents Canonical/Dedup Tests
// ============================================================================

/// Test list_canonical returns documents (with fallback for missing is_canonical column)
#[tokio::test]
async fn test_list_canonical_documents() {
    let pool = create_pool(DEFAULT_DATABASE_URL).await.expect("Pool creation failed");
    
    let docs = documents::list_canonical(&pool, 10, 0).await;
    assert!(docs.is_ok(), "list_canonical failed: {:?}", docs.err());
    
    let docs = docs.unwrap();
    assert!(!docs.is_empty(), "Expected canonical documents but got empty list");
    
    // Verify basic field presence
    let first = &docs[0];
    assert!(first.id > 0, "Document should have positive id");
    assert!(!first.path.is_empty(), "Document should have a path");
}

/// Test search_canonical searches within canonical documents
#[tokio::test]
async fn test_search_canonical_documents() {
    let pool = create_pool(DEFAULT_DATABASE_URL).await.expect("Pool creation failed");
    
    // Search for a common term that should exist in documents
    let results = documents::search_canonical(&pool, "journal", 10).await;
    assert!(results.is_ok(), "search_canonical failed: {:?}", results.err());
    
    let results = results.unwrap();
    // We expect at least some results for common search terms
    // Note: even if empty, the function should succeed
    if !results.is_empty() {
        let first = &results[0];
        assert!(first.id > 0, "Document should have positive id");
    }
}

/// Test get_all_titles_canonical returns titles for autocomplete
#[tokio::test]
async fn test_get_all_titles_canonical() {
    let pool = create_pool(DEFAULT_DATABASE_URL).await.expect("Pool creation failed");
    
    let titles = documents::get_all_titles_canonical(&pool).await;
    assert!(titles.is_ok(), "get_all_titles_canonical failed: {:?}", titles.err());
    
    let titles = titles.unwrap();
    assert!(!titles.is_empty(), "Expected titles but got empty list");
    
    // We have ~47K documents, so should have many titles
    assert!(titles.len() > 1000, "Expected >1000 titles, got {}", titles.len());
    
    // Verify tuple structure
    let (id, title) = &titles[0];
    assert!(*id > 0, "ID should be positive");
    assert!(!title.is_empty(), "Title should not be empty");
}

/// Test get_versions returns empty when no canonical_id column exists yet
#[tokio::test]
async fn test_get_versions() {
    let pool = create_pool(DEFAULT_DATABASE_URL).await.expect("Pool creation failed");
    
    // This should succeed even if the column doesn't exist (returns empty vec)
    let versions = documents::get_versions(&pool, 1).await;
    assert!(versions.is_ok(), "get_versions should not error: {:?}", versions.err());
    
    // If the column doesn't exist yet, we get an empty vec (which is fine)
    // This is the expected fallback behavior
}

/// Test documents list_light returns document metadata
#[tokio::test]
async fn test_list_documents() {
    let pool = create_pool(DEFAULT_DATABASE_URL).await.expect("Pool creation failed");
    
    let docs = documents::list_light(&pool, 10, 0).await;
    assert!(docs.is_ok(), "list_light failed: {:?}", docs.err());
    
    let docs = docs.unwrap();
    assert!(!docs.is_empty(), "Expected documents but got empty list");
    
    let first = &docs[0];
    assert!(first.id > 0, "Document should have positive id");
    assert!(!first.path.is_empty(), "Document should have a path");
}

/// Test documents get_count returns a large count
#[tokio::test]
async fn test_documents_count() {
    let pool = create_pool(DEFAULT_DATABASE_URL).await.expect("Pool creation failed");
    
    let count = documents::get_count(&pool).await;
    assert!(count.is_ok(), "get_count failed: {:?}", count.err());
    
    let count = count.unwrap();
    // We have ~47K documents
    assert!(count > 40000, "Expected >40000 documents, got {}", count);
}

/// Test documents search functionality
#[tokio::test]
async fn test_search_documents() {
    let pool = create_pool(DEFAULT_DATABASE_URL).await.expect("Pool creation failed");

    let results = documents::search(&pool, "music", 10).await;
    assert!(results.is_ok(), "search failed: {:?}", results.err());

    let results = results.unwrap();
    // Music-related docs should exist in the archive
    assert!(!results.is_empty(), "Expected search results for 'music'");
}

// ============================================================================
// Tasks Tests
// ============================================================================

use super::tasks;
use chrono::NaiveDate;

/// Test getting tasks due on a specific date
#[tokio::test]
async fn test_get_tasks_due_on() {
    let pool = create_pool(DEFAULT_DATABASE_URL).await.expect("Pool creation failed");

    // Use a date that's likely to have tasks (today)
    let today = chrono::Local::now().date_naive();

    let tasks = tasks::get_tasks_due_on(&pool, today).await;
    assert!(tasks.is_ok(), "get_tasks_due_on failed: {:?}", tasks.err());

    let tasks = tasks.unwrap();
    // May be empty if no tasks due today, that's fine
    for task in &tasks {
        assert!(task.id > 0, "Task should have positive id");
        assert!(!task.text.is_empty(), "Task should have text");
        // Verify due date matches
        if let Some(ref due) = task.due_date {
            assert_eq!(due, &today.format("%Y-%m-%d").to_string(), "Due date should match query");
        }
    }
}

/// Test getting open tasks due on a specific date
#[tokio::test]
async fn test_get_open_tasks_due_on() {
    let pool = create_pool(DEFAULT_DATABASE_URL).await.expect("Pool creation failed");

    let today = chrono::Local::now().date_naive();

    let tasks = tasks::get_open_tasks_due_on(&pool, today).await;
    assert!(tasks.is_ok(), "get_open_tasks_due_on failed: {:?}", tasks.err());

    let tasks = tasks.unwrap();
    // Verify all returned tasks are open
    for task in &tasks {
        assert!(task.is_open(), "All tasks should be open, but got status: {}", task.status);
    }
}

/// Test getting all open tasks
#[tokio::test]
async fn test_get_all_open_tasks() {
    let pool = create_pool(DEFAULT_DATABASE_URL).await.expect("Pool creation failed");

    let tasks = tasks::get_all_open_tasks(&pool).await;
    assert!(tasks.is_ok(), "get_all_open_tasks failed: {:?}", tasks.err());

    let tasks = tasks.unwrap();
    // Should have at least some open tasks
    // Verify all returned tasks are open
    for task in &tasks {
        assert!(task.is_open(), "All tasks should be open, but got status: {}", task.status);
        assert!(task.id > 0, "Task should have positive id");
        assert!(!task.text.is_empty(), "Task should have text");
    }
}

/// Test getting overdue tasks
#[tokio::test]
async fn test_get_overdue_tasks() {
    let pool = create_pool(DEFAULT_DATABASE_URL).await.expect("Pool creation failed");

    let today = chrono::Local::now().date_naive();

    let tasks = tasks::get_overdue_tasks(&pool, today).await;
    assert!(tasks.is_ok(), "get_overdue_tasks failed: {:?}", tasks.err());

    let tasks = tasks.unwrap();
    // Verify all returned tasks are overdue and open
    for task in &tasks {
        assert!(task.is_open(), "Overdue tasks should be open");
        if let Some(ref due) = task.due_date {
            let due_date = NaiveDate::parse_from_str(due, "%Y-%m-%d")
                .expect("Due date should be valid");
            assert!(due_date < today, "Task should be overdue");
        }
    }
}

/// Test task formatting for daily note insertion
#[tokio::test]
async fn test_format_tasks_for_daily_note() {
    let pool = create_pool(DEFAULT_DATABASE_URL).await.expect("Pool creation failed");

    let today = chrono::Local::now().date_naive();
    let tasks = tasks::get_open_tasks_due_on(&pool, today).await
        .expect("Failed to get tasks");

    let formatted = tasks::format_tasks_for_daily_note(&tasks);

    if !tasks.is_empty() {
        assert!(!formatted.is_empty(), "Formatted output should not be empty");
        assert!(formatted.contains("- ["), "Should contain checkbox format");
    } else {
        assert!(formatted.is_empty(), "Empty task list should produce empty string");
    }
}
