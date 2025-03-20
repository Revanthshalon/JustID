CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS relationships (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  namespace VARCHAR(255) NOT NULL,
  object_id VARCHAR(255) NOT NULL,
  relation VARCHAR(255) NOT NULL,
  subject_namespace VARCHAR(255) NOT NULL,
  subject_id  VARCHAR(255) NOT NULL,
  subject_relation VARCHAR(255) NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  UNIQUE (namespace, object_id, relation, subject_namespace, subject_id, subject_namespace)
);

CREATE TABLE IF NOT EXISTS namespaces (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  name VARCHAR(255) NOT NULL UNIQUE,
  relation_definitions JSONB NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS permission_definitions (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  namespace VARCHAR(255) NOT NULL,
  permission VARCHAR(255) NOT NULL,
  relation_expressions JSONB NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  UNIQUE (namespace, permission)
);

CREATE TABLE IF NOT EXISTS authorization_logs (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  timestamp TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  subject_id VARCHAR(255) NOT NULL,
  resource VARCHAR(255) NOT NULL,
  permission VARCHAR(255) NOT NULL,
  granted BOOLEAN NOT NULL,
  context JSONB
);

CREATE INDEX idx_relationships_object ON relationships (namespace, object_id, relation);
CREATE INDEX idx_relationships_subject ON relationships (subject_namespace, subject_id);
CREATE INDEX idx_relationships_subject_with_relation ON relationships (subject_namespace, subject_id, subject_relation) WHERE subject_relation IS NOT NULL;

CREATE OR REPLACE FUNCTION update_timestamp()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = CURRENT_TIMESTAMP;
  return NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_relationships_timestamp
BEFORE UPDATE ON relationships
FOR EACH ROW EXECUTE FUNCTION update_timestamp();

CREATE TRIGGER update_namespaces_timestamp
BEFORE UPDATE ON namespaces
FOR EACH ROW EXECUTE FUNCTION update_timestamp();

CREATE TRIGGER update_permission_definitions_timestamp
BEFORE UPDATE ON permission_definitions
FOR EACH ROW EXECUTE FUNCTION update_timestamp();
