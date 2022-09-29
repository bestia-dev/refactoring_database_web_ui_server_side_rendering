create or replace function public.webpage_hits_update(
_id int,
_webpage varchar(100),
_hit_count integer)
returns table(id integer, webpage varchar(100), hit_count integer) 
language 'plpgsql'
as $body$
declare
begin

update webpage as w
set webpage = _webpage
where w.id = _id;

update hit_counter as h
set count = _hit_count
where h.webpage_id=_id;

return query 
select W.id, W.webpage, W.hit_count
from webpage_hits W
where W.id=_id;

end; 
$body$;
