-- This file should undo anything in `up.sql`

DROP FUNCTION IF EXISTS shortkey_generate();

DROP DOMAIN IF EXISTS SHORTKEY;