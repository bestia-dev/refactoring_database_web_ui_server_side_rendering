create or replace view public.get_view_fields
as
-- select * from get_view_fields ;
-- types: int4, varchar, name, text,...

SELECT
    c.relname,
    a.attname,
    t.typname
FROM pg_class c
INNER JOIN pg_attribute a ON a.attrelid = c.oid
INNER JOIN pg_type t ON t.oid = a.atttypid
WHERE c.relkind = 'v'
and c.relnamespace not in (11,13161)