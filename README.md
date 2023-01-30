#### PostgreSQL Using `diesel`

Verify PostgreSQL

```bash
psql --dbname=postgres --host=localhost --port=5432 --username=postgres
```

```bash
cargo install diesel_cli --no-default-features --features postgres
```

```bash
echo DATABASE_URL=postgres://postgres:test123@localhost/mydb > .env
```

```bash
diesel setup
```

After Running `diesel setup`

```sql
postgres=# \l
                                                List of databases
   Name    |  Owner   | Encoding |  Collate   |   Ctype    | ICU Locale | Locale Provider |   Access privileges
-----------+----------+----------+------------+------------+------------+-----------------+-----------------------
 mydb      | postgres | UTF8     | en_US.utf8 | en_US.utf8 |            | libc            |
```

Connect to `mydb` PostgreSQL Database

```sql
postgres=# \c mydb
You are now connected to database "mydb" as user "postgres".

mydb=# \d
                   List of relations
 Schema |            Name            | Type  |  Owner
--------+----------------------------+-------+----------
 public | __diesel_schema_migrations | table | postgres
(1 row)
```

```bash
diesel migration generate create_books
```

Output

```bash
Creating migrations/2023-01-30-000542_create_books/up.sql
Creating migrations/2023-01-30-000542_create_books/down.sql
```

Populate the `*.sql` file with the below contents

Contents of `migrations/2023-01-30-000542_create_books/up.sql`

```sql
-- Your SQL goes here

CREATE TABLE books (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    author VARCHAR NOT NULL,
    published BOOLEAN NOT NULL DEFAULT 'f'
);
```

Contents of `migrations/2023-01-30-000542_create_books/down.sql`

```sql
DROP TABLE books;
```

```bash
diesel migration run
```

```sql
mydb=# \d
                     List of relations
 Schema |            Name            |   Type   |  Owner
--------+----------------------------+----------+----------
 public | __diesel_schema_migrations | table    | postgres
 public | books                      | table    | postgres
 public | books_id_seq               | sequence | postgres
(3 rows)

mydb=# \d books;
                                   Table "public.books"
  Column   |       Type        | Collation | Nullable |              Default
-----------+-------------------+-----------+----------+-----------------------------------
 id        | integer           |           | not null | nextval('books_id_seq'::regclass)
 title     | character varying |           | not null |
 author    | character varying |           | not null |
 published | boolean           |           | not null | false
Indexes:
    "books_pkey" PRIMARY KEY, btree (id)
```

To `redo` the migrations (It will remove the table and recreate the table)

```bash
diesel migration redo
```

```bash
diesel print-schema
```

Output

```rust
// @generated automatically by Diesel CLI.

diesel::table! {
    books (id) {
        id -> Int4,
        title -> Varchar,
        author -> Varchar,
        published -> Bool,
    }
}
```

Create `src/schema.rs`

```bash
diesel print-schema > src/schema.rs
```