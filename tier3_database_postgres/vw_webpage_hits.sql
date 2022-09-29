create or replace view public.webpage_hits
as

select w.id,
w.webpage,
h.count as hit_count
from webpage w
join hit_counter h on h.webpage_id = w.id
where w.id = h.webpage_id
order by w.webpage;
