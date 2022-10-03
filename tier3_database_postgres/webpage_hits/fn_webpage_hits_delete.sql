create or replace function public.webpage_hits_delete(
_id integer)
returns void
language 'plpgsql'
as $body$
declare
begin

delete from hit_counter h
where h.webpage_id = _id;

delete from webpage w
where w.id = _id;

end; 
$body$;