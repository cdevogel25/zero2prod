-- Add migration script here
-- wrap the whole migration in a transaction!
-- this way it succeeds or fails atomically (you should learn more about this)
-- `sqlx` does not automatically do this
BEGIN;
    -- backfill `status` for historical entries
    UPDATE subscriptions
        SET status = 'confirmed'
        WHERE status IS NULL;
    -- make `status` mandatory
    ALTER TABLE subscriptions ALTER COLUMN status SET NOT NULL;
COMMIT;