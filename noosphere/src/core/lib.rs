//! dpn-core: Shared infrastructure for DPN tools ecosystem
//!
//! This crate provides:
//! - Database connections and models (PostgreSQL via SSH tunnel)
//! - ICS calendar parsing
//! - Stagehand notes management for show/venue tracking
//! - Sync layer for local ↔ remote database synchronization
//! - Agent memory storage and cross-agent recall
//! - Embedding generation and semantic search
//! - Smart context injection
//! - Local SQLite cache for offline-first access (~/.dpn/cache.db)
//! - Document deduplication tools
//! - Webhook notifications for EM Staff

pub mod db;
pub mod ics;
pub mod stagehand;
pub mod sync;
pub mod memory;
pub mod embeddings;
pub mod context;
pub mod cache;
pub mod dedup;
pub mod notify;
pub mod pipeline;
pub mod tasks;
pub mod events;
pub mod reading;
pub mod wikilinks;
pub mod graph;
pub mod timeline;
pub mod conversations;
// TODO: Uncomment after Phase 6 - publish module requires streams/drops tables
// pub mod publish;

// Re-exports for convenience
pub use db::{DbPool, create_pool};
pub use db::memories::{Memory, MemoryLight};
pub use db::stagehand::{StagehandNote, StagehandNoteCreate};
pub use db::tasks::{Task, get_tasks_due_on, get_open_tasks_due_on, get_overdue_tasks};
pub use db::tasks::{get_all_open_tasks, format_tasks_for_daily_note};
pub use db::events::{Event, get_events_on_date, get_events_in_range, get_upcoming_events};
pub use db::events::{get_event_by_id, upsert_event, delete_event, count_events_on_date};
pub use db::projects::{Project, list_projects, list_active_projects, get_project_by_id};
pub use db::projects::{get_project_by_slug, create_project, update_project, update_project_status, delete_project};

// Areas exports (PARAT pillar)
pub use db::areas::{Area, list_areas, get_area_by_id, create_area, update_area};

// Archives exports (PARAT pillar)
pub use db::archives::{Archive, list_archives, get_archive_by_id, create_archive, update_archive_metadata, search_archives};

// Resources exports (PARAT pillar)
pub use db::resources::{Resource, list_resources, get_resource_by_id, create_resource, update_resource};

// Templates exports (PARAT pillar)
pub use db::templates::{Template, TemplateHistory, list_templates, get_template_by_id, create_template, update_template, get_template_history};

pub use db::documents::{Document, DocumentMeta, DocumentVersion};
pub use db::documents::{get_by_id as get_document_by_id, get_by_path as get_document_by_path};
pub use db::documents::{list_light as list_documents, search as search_documents};
pub use db::documents::{list_canonical, search_canonical, get_all_titles_canonical, get_versions};
pub use db::documents::{create_document, update_document, delete_document};
pub use ics::parser::{IcsEvent, parse_ics};

// Task parser exports
pub use tasks::{Task as ParsedTask, TaskStatus, TaskPriority};
pub use tasks::{parse_document as parse_tasks, parse_task_line, serialize_task};

// Event parser exports
pub use events::{ParsedEvent, EventType};
pub use events::{parse_events, parse_event_line, is_weekly_note};

// Sync exports
pub use sync::{IncrementalSync, SyncState, SnapshotSync, ConflictStrategy, ConflictResolver};

// Memory exports
pub use memory::{DailyLog, DailyLogCreate, MemoryEntry, MemoryEntryCreate, Agent};
pub use memory::{write_log, write_memory, list_agents, get_agent};
pub use memory::recall::{RecallResult, SemanticRecall};
pub use memory::inheritance::MemoryInheritance;

// Embedding exports
pub use embeddings::{EmbeddingConfig, EmbeddingService, EmbeddingProvider, generate_embedding};
pub use embeddings::{backfill_embeddings, BackfillProgress, BackfillTarget};
pub use embeddings::{write_log_with_embedding, write_memory_with_embedding, create_stagehand_note_with_embedding};

// Context exports
pub use context::{get_related_context, ContextResult, ContextSource};
pub use context::{RelevanceScorer, ScoredResult};

// Cache exports (offline-first local storage)
pub use cache::{CachePool, init_cache, get_cache_path};
pub use cache::{PendingChange, ChangeOperation, SyncQueue};
pub use cache::HybridStore;

// Dedup exports (document deduplication)
pub use dedup::{DuplicateCluster, DocInfo, MigrationSummary, MigrationResult};
pub use dedup::{find_duplicates_by_title, select_canonical, dry_run_all, migrate_all};
pub use dedup::{LocationPriority, DuplicateStats};

// Notify exports (EM Staff webhook notifications)
pub use notify::{NotifyClient, NotifyConfig, NotifyError, Office, Phase};
pub use notify::{staff_display_name, PHASE_MAPPINGS, get_office_phases};

// Pipeline exports (automated workflows)
pub use pipeline::{insert_tasks_due_into_daily_note, InsertionResult, InsertionMode};

// Reading exports (RSS reader + comments)
pub use reading::{Feed, Article, FirehoseArticle, ReadingComment};
pub use reading::{subscribe_feed, list_feeds, get_feed, refresh_feed, get_firehose, mark_read, create_comment, import_opml};
pub use reading::{discover_feeds, is_feed_url, subscribe_to_url, fetch_feed};

// Wikilink exports ([[Note Title]] parsing and resolution)
pub use wikilinks::{Wikilink, ResolvedWikilink, WikilinkResolution};
pub use wikilinks::{parse_wikilinks, resolve_wikilink, resolve_all_wikilinks};
pub use wikilinks::{extract_unique_targets, wikilinks_to_html, build_link_graph};

// Graph visualization exports (node/edge data for document relationships)
pub use graph::{GraphNode, GraphEdge, GraphData, GraphStats, GraphOptions};
pub use graph::{build_graph, build_neighborhood_graph, get_hub_documents, get_orphan_documents};

// Timeline exports (unified chronological view across data sources)
pub use timeline::{TimelineEntry, TimelineType};
pub use timeline::{load_timeline_entries, get_activity_counts};

// Conversations exports (agent-to-agent messaging)
pub use conversations::{Conversation, ConversationCreate, ConversationLight};
pub use conversations::{send_message, send_message_full, get_thread, get_unread};
pub use conversations::{mark_read as mark_conversation_read, count_unread, mark_thread_read};
pub use conversations::{get_by_id as get_conversation_by_id};
pub use conversations::{get_recent as get_recent_conversations, get_by_channel, get_direct};
pub use conversations::{list_threads, get_thread_summaries};

// Publish exports (RSS-first publishing) - Disabled until Phase 6
// pub use publish::{Stream, StreamCreate, Drop as PublishDrop, DropCreate, DropUpdate, DropStatus, DropFilter};
// pub use publish::{ThoughtPoliceResponse, ResponseCreate};
// pub use publish::{create_stream, get_stream, get_stream_by_slug, list_streams, update_stream, delete_stream};
// pub use publish::{create_drop, get_drop, get_drop_by_slug, list_drops, list_published_drops};
// pub use publish::{update_drop, delete_drop, publish_drop};
// pub use publish::{create_response, list_responses, moderate_response, delete_response};
// pub use publish::{generate_rss, generate_atom, generate_podcast_rss, FeedFormat};
// pub use publish::{markdown_to_html, extract_frontmatter, auto_excerpt, Frontmatter};
