CREATE OR REPLACE FUNCTION drop_function(_name text, OUT functions_dropped int)
   LANGUAGE plpgsql AS
$func$
-- drop all functions with given _name regardless of function parameters
-- test it: create function test1. Then 
-- select drop_function('webpage_hits_delete');
DECLARE
   _sql text;
BEGIN
   SELECT count(*)::int
        , 'DROP FUNCTION ' || string_agg(oid::regprocedure::text, '; DROP FUNCTION ')
   FROM   pg_catalog.pg_proc
   WHERE  proname = _name
   AND    pg_function_is_visible(oid)  -- restrict to current search_path
   INTO   functions_dropped, _sql;     -- count only returned if subsequent DROPs succeed

   IF functions_dropped > 0 THEN       -- only if function(s) found
     EXECUTE _sql;
   END IF;
END
$func$;