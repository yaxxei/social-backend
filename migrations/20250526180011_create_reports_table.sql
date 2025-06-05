-- Add migration script here
CREATE TYPE report_target_type AS ENUM ('post', 'comment', 'user');
CREATE TYPE report_status_type AS ENUM ('pending', 'approved', 'rejected', 'processed');

CREATE TABLE reports (
    id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
    report_type report_target_type NOT NULL,
    reported_id UUID NOT NULL,
    reporter_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    reason TEXT,
    status report_status_type DEFAULT 'pending',
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TRIGGER set_updated_at 
BEFORE UPDATE ON reports 
FOR EACH ROW EXECUTE 
FUNCTION update_updated_at_column();

