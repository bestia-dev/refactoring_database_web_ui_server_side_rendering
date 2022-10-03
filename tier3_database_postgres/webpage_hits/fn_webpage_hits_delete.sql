create or replace function public.webpage_hits_delete(
_id integer)
returns table(deleted_rows integer) 
language 'plpgsql'
as $body$
declare
begin

delete from hit_counter h
where h.webpage_id = _id;

delete from webpage w
where w.id = _id;

return query 
select 1 as deleted_rows;

end; 
$body$;