-- DEV ONLY - Brute Force DROP BD (for local dev and unit test)
SELECT pg_terminate_backend(pid)
FROM pg_catalog.pg_stat_activity
WHERE pg_stat_activity.usename = 'app_user'
   OR pg_stat_activity.datname = 'app_db';

DROP DATABASE IF EXISTS app_db;
DROP USER IF EXISTS app_user;

-- DEV ONLY - Dev only password (for local dev and unit test)
CREATE USER app_user PASSWORD 'dev_only_pwd';
CREATE DATABASE app_db owner app_user ENCODING = 'UTF-8';