# from-psql-row

_A simple crate to convert from [tokio_postgres::Row](https://docs.rs/tokio-postgres/latest/tokio_postgres/row/struct.Row.html) to a struct_. It's practically like [serde_postgres](https://docs.rs/serde_postgres/latest/serde_postgres/), except **dumber**, and **simpler**.

---


**What does it do?**

Nothing fancy, just creates an `impl TryFrom<tokio_postgres::Row> for MyStruct` if `MyStruct` is what you started with.

**Why should I use this, if `serde_postgres` is available?**

Because it's so SIMPLE. `serde_postgres` needs your struct to have `#[derive(Deserialize)]`, meaning `serde` is needed. If for some reason all you want is to create a struct from `Row` without having to manually populate fields using `row.get::<_, T>("columnName")` for each column, while avoiding `serde` (and keeping a minimal dependency footprint), you're in the right place.

#### Features
- First off, the `impl`. (Duh!) 
- Works for any data type that implements [tokio_postgres::FromSql](https://docs.rs/tokio-postgres/latest/tokio_postgres/types/trait.FromSql.html) trait.
- Can handle fields with reserved keyword names. E.g., `r#type` maps to column "**type**" inside the `Row`.
- Struct fields can be aliased to different column names. E.g., `user_name` can be aliased to column `user`. Check usage.

#### Usage
```rust
use from_psql_row::FromPsql;

#[derive(FromPsql)]
struct MyStruct {
    #[sqlfield("userId")] // Maps to column 'userId', not 'user_name'
    user_name: Uuid,
    r#type: Option<String>, // Maps to column 'type'
    age: u8,
}
```
---
To use it in your project, run
```
cargo add from-psql-row --git https://github.com/ByteBaker/from-psqlrow-rs
```
or just add the following line to your `Cargo.toml`
```toml
from-psql-row = { version = "0.1.0", git = "https://github.com/ByteBaker/from-psqlrow-rs" }
```
