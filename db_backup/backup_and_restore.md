# Backup and restore for database webpage_hit_counter

## Backup

Run this from the VSCode terminal inside the project folder.

```bash
pg_dump -F t -U admin -h localhost -p 5432 webpage_hit_counter > db_backup/webpage_hit_counter.tar
```

## Restore

Run this from the VSCode terminal inside the project folder.

```bash
createdb -U admin -h localhost -p 5432 webpage_hit_counter2; pg_restore -c -U admin -h localhost -p 5432 -d webpage_hit_counter2 db_backup/webpage_hit_counter.tar
```

## psql connection

psql is the command line utility for managing postgres.  
It is very effective.
Auto-completion works ! But not for fields in a table.
History works !
Every sql statement must end with semicolon !
If the result is long, use PgUp, PgDn, End, Home keys to scroll,
then exit scroll with "\q".

```bash
psql -U admin -h localhost -p 5432 -d webpage_hit_counter
```

```psql
\l     List database
\c     Current database
\c dbname   Switch connection to a new database
\dt    List tables
\dv    List views
\df    List functions
\q     Exit psql shell

-- every sql statement must end with semicolon !
select * from webpage;
select * from hit_counter h;

```
