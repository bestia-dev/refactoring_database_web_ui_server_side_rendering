create or replace function public.random_between(
low integer,
high integer)
returns integer
language 'plpgsql'
strict
as $body$
begin

return floor(random()* (high-low + 1) + low);

end;
$body$;
