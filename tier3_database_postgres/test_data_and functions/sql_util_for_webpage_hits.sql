
-- the new "database object" is webpage_hits

select * from webpage_hits;

-- crud create(insert), read (show, list), update, delete
-- but we need 1 more for the new records with defaults

-- returns the dataset of inserted record

select id, webpage, hit_count from webpage_hits_new();

select *
from webpage_hits_insert(1234,'test333',333);

select *
from webpage_hits_show(1234);

select *
from webpage_hits_update(1234,'1234', 1234);

select webpage_hits_delete(1234);

-- overloading functions in postgres is abominable
-- check and drop the duplicates

select count(*), proname 
from pg_proc 
where pronamespace <> 11 
group by proname 
having count(*) > 1;

-- drop function

select * from function_params;


select p.proname, count(*) as cnt
FROM pg_proc p 
group by p.proname;

-- format_type(type_oid, typemod)	text	get SQL name of a data type

-- pg_get_function_arguments(func_oid)

select * FROM pg_proc;