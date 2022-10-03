create or replace view public.webpage_hits_list
as
-- select * from webpage_hits_list;

select w.id, w.webpage, w.hit_count
from webpage_hits w;
