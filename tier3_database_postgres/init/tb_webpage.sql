
CREATE TABLE IF NOT EXISTS public.webpage
(
    id integer NOT NULL,
    webpage varchar(100) COLLATE pg_catalog."default" NOT NULL,
    CONSTRAINT webpage_pkey PRIMARY KEY (id),
    CONSTRAINT webpage_uniq_webpage UNIQUE (webpage)
)
