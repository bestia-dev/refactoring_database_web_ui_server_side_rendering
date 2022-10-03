create or replace function public.webpage_hits_insert(
_id integer,
_webpage varchar(100),
_hit_count integer)
returns table(id integer, webpage varchar(100), hit_count integer) 
language 'plpgsql'
as $body$
declare

begin

insert into webpage ( "id", webpage)
values (_id, _webpage);

insert into hit_counter(webpage_id, "count")
values(_id,_hit_count);

return query 
select w.id, w.webpage, w.hit_count
from webpage_hits w
where w.id=_id;

end; 
$body$;
