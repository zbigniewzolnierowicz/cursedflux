-- This file should undo anything in `up.sql`

DROP TRIGGER IF EXISTS trigger_test_genid ON users;

DROP TABLE IF EXISTS users CASCADE;