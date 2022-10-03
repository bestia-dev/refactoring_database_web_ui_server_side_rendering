create or replace function public.webpage_hits_new()
returns table(id integer, webpage varchar(100), hit_count integer) 
language 'plpgsql'
as $body$
declare
begin
return query 

-- the id is random from one billion. that is enough for my simple tutorial.
select random_between(1, 1000000000) as id, 
'webpage short url'::varchar(100) as webpage, 
0 as hit_count;

end; 
$body$;
