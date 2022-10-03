create or replace view public.get_function_input_params
as
-- select * from get_function_input_params ;

SELECT 
p.proname, 
-- definition of arguments without defaults. 
-- returns string that must be parsed later.
-- _id integer, _webpage character varying, _hit_count integer
pg_get_function_arguments(oid) AS args_def 
FROM pg_proc p 
-- don't show system functions
where p.pronamespace not in (11,13161)
order by p.proname;
