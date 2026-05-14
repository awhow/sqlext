
This workspace defines a ToRow trait and a ToRow proc macro. See the example below:


Your code may use the ToRow trait like this:

```insert.rs
use async_trait::async_trait;
use sqlext::ToRow;
use sqlx::{Pool, Sqlite};

#[async_trait]
pub trait Insert: ToRow<Sqlite> + Send + Sync {
    async fn insert(&self, pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
        let columns = <Self as ToRow<Sqlite>>::columns();

        let placeholders = (0..columns.len())
            .map(|_| "?")
            .collect::<Vec<_>>()
            .join(", ");

        let sql = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            <Self as ToRow<Sqlite>>::table_name(),
            columns.join(", "),
            placeholders,
        );

        let query = sqlx::query(&sql);
        let query = <Self as ToRow<Sqlite>>::bind(self, query);
        query.execute(pool).await?;

        Ok(())
    }
}

```

Then your structs may use the ToRow macro like this:


```emp.rs
use std::fmt::Display;

use sqlext_derive::ToRow;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow, Debug, Clone, ToRow)]
pub struct Employee {
    pub id: Uuid,
    pub name: String,
}

impl Insert for Employee {}
```


Now in your application code, you can just run:

```main.rs

use crate::Insert;

let emp = Employee::new()
emp.insert(pool).await?;

```

There is some flexibility built in

```rust
#[derive(FromRow, Debug, Clone, ToRow)]
#[sqlext(table = "employees")]
pub struct Employee {
    pub id: Uuid,

    #[sqlext(column = "employee_name")]
    pub name: String,

    #[sqlext(skip)]
    pub nick_name: String,
}
```
