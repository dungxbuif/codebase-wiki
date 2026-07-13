-- CodeWiki durable local state schema.
-- This migration is executor-agnostic SQL intended for SQLite.

PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS schema_migrations (
  version INTEGER PRIMARY KEY,
  name TEXT NOT NULL,
  checksum TEXT NOT NULL,
  applied_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS repositories (
  id TEXT PRIMARY KEY,
  root_path TEXT NOT NULL,
  git_remote TEXT,
  current_branch TEXT,
  git_head TEXT,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS sync_runs (
  id TEXT PRIMARY KEY,
  repository_id TEXT NOT NULL,
  mode TEXT NOT NULL,
  status TEXT NOT NULL,
  started_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  finished_at TEXT,
  git_head_before TEXT,
  git_head_after TEXT,
  model_id TEXT,
  notes TEXT,
  FOREIGN KEY (repository_id) REFERENCES repositories(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS files (
  id TEXT PRIMARY KEY,
  repository_id TEXT NOT NULL,
  path TEXT NOT NULL,
  content_hash TEXT,
  language TEXT,
  role TEXT NOT NULL DEFAULT 'source',
  is_generated INTEGER NOT NULL DEFAULT 0,
  last_seen_run_id TEXT,
  updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (repository_id) REFERENCES repositories(id) ON DELETE CASCADE,
  FOREIGN KEY (last_seen_run_id) REFERENCES sync_runs(id) ON DELETE SET NULL,
  UNIQUE (repository_id, path)
);

CREATE TABLE IF NOT EXISTS symbols (
  id TEXT PRIMARY KEY,
  repository_id TEXT NOT NULL,
  file_id TEXT NOT NULL,
  name TEXT NOT NULL,
  kind TEXT NOT NULL,
  signature TEXT,
  start_line INTEGER,
  end_line INTEGER,
  confidence TEXT NOT NULL DEFAULT 'source-backed',
  FOREIGN KEY (repository_id) REFERENCES repositories(id) ON DELETE CASCADE,
  FOREIGN KEY (file_id) REFERENCES files(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS pages (
  id TEXT PRIMARY KEY,
  repository_id TEXT NOT NULL,
  path TEXT NOT NULL,
  title TEXT NOT NULL,
  canonical_slot TEXT,
  content_hash TEXT,
  status TEXT NOT NULL DEFAULT 'draft',
  last_generated_run_id TEXT,
  human_owned_regions TEXT,
  updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (repository_id) REFERENCES repositories(id) ON DELETE CASCADE,
  FOREIGN KEY (last_generated_run_id) REFERENCES sync_runs(id) ON DELETE SET NULL,
  UNIQUE (repository_id, path)
);

CREATE TABLE IF NOT EXISTS evidence_items (
  id TEXT PRIMARY KEY,
  repository_id TEXT NOT NULL,
  kind TEXT NOT NULL,
  source_path TEXT,
  symbol_id TEXT,
  command TEXT,
  summary TEXT NOT NULL,
  content_hash TEXT,
  confidence TEXT NOT NULL DEFAULT 'source-backed',
  observed_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  run_id TEXT,
  FOREIGN KEY (repository_id) REFERENCES repositories(id) ON DELETE CASCADE,
  FOREIGN KEY (symbol_id) REFERENCES symbols(id) ON DELETE SET NULL,
  FOREIGN KEY (run_id) REFERENCES sync_runs(id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS claims (
  id TEXT PRIMARY KEY,
  repository_id TEXT NOT NULL,
  page_id TEXT,
  statement TEXT NOT NULL,
  status TEXT NOT NULL DEFAULT 'active',
  confidence TEXT NOT NULL DEFAULT 'source-backed',
  owner TEXT NOT NULL DEFAULT 'ai',
  first_seen_run_id TEXT,
  last_verified_run_id TEXT,
  updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (repository_id) REFERENCES repositories(id) ON DELETE CASCADE,
  FOREIGN KEY (page_id) REFERENCES pages(id) ON DELETE SET NULL,
  FOREIGN KEY (first_seen_run_id) REFERENCES sync_runs(id) ON DELETE SET NULL,
  FOREIGN KEY (last_verified_run_id) REFERENCES sync_runs(id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS claim_evidence (
  claim_id TEXT NOT NULL,
  evidence_id TEXT NOT NULL,
  relationship TEXT NOT NULL DEFAULT 'supports',
  PRIMARY KEY (claim_id, evidence_id),
  FOREIGN KEY (claim_id) REFERENCES claims(id) ON DELETE CASCADE,
  FOREIGN KEY (evidence_id) REFERENCES evidence_items(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS provider_snapshots (
  id TEXT PRIMARY KEY,
  repository_id TEXT NOT NULL,
  provider TEXT NOT NULL,
  version TEXT,
  config_hash TEXT,
  trigger_reason TEXT NOT NULL,
  captured_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  summary TEXT,
  FOREIGN KEY (repository_id) REFERENCES repositories(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS open_questions (
  id TEXT PRIMARY KEY,
  repository_id TEXT NOT NULL,
  question TEXT NOT NULL,
  status TEXT NOT NULL DEFAULT 'active',
  owner TEXT NOT NULL DEFAULT 'unknown',
  source TEXT,
  created_run_id TEXT,
  resolved_run_id TEXT,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  resolved_at TEXT,
  FOREIGN KEY (repository_id) REFERENCES repositories(id) ON DELETE CASCADE,
  FOREIGN KEY (created_run_id) REFERENCES sync_runs(id) ON DELETE SET NULL,
  FOREIGN KEY (resolved_run_id) REFERENCES sync_runs(id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_sync_runs_repository ON sync_runs(repository_id, started_at);
CREATE INDEX IF NOT EXISTS idx_files_repository_path ON files(repository_id, path);
CREATE INDEX IF NOT EXISTS idx_symbols_repository_name ON symbols(repository_id, name);
CREATE INDEX IF NOT EXISTS idx_pages_repository_path ON pages(repository_id, path);
CREATE INDEX IF NOT EXISTS idx_evidence_repository_kind ON evidence_items(repository_id, kind);
CREATE INDEX IF NOT EXISTS idx_claims_repository_status ON claims(repository_id, status);
CREATE INDEX IF NOT EXISTS idx_open_questions_repository_status ON open_questions(repository_id, status);
