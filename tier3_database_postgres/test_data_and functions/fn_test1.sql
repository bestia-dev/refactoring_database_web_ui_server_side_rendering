CREATE OR REPLACE FUNCTION test1(_name text)
returns table(oid integer) 
   LANGUAGE plpgsql AS
$func$
-- just a test function so I can test how to drop it
-- with drop_function
DECLARE

BEGIN
  return query 
select oid FROM pg_proc p;
END
$func$;