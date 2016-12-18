-- -- complain if script is sourced in psql, rather than via CREATE EXTENSION
\echo Use "CREATE EXTENSION rust_base36" to load this file. \quit
CREATE FUNCTION base36_encode(INT) RETURNS TEXT
  AS '$libdir/rpg_base36'
LANGUAGE C IMMUTABLE STRICT;