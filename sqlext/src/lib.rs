use sqlx::database::Database;
use sqlx::query::Query;

pub trait ToRow<DB>
where
    DB: Database,
{
    /// Name of table in SQL database
    fn table_name() -> &'static str;

    /// List of all column names for table
    ///
    /// `columns()` = `pkey_columns()` + `data_columns()`
    fn columns() -> &'static [&'static str];

    /// SQL literal values for all columns.
    ///
    /// `values()` = `pkey_values()` + `data_values()`
    fn values(&self) -> Vec<String>;

    /// Query binding for all columns
    fn bind<'q>(
        &'q self,
        query: Query<'q, DB, DB::Arguments<'q>>,
    ) -> Query<'q, DB, DB::Arguments<'q>>;

    /// List of primary key column names
    fn pkey_columns() -> &'static [&'static str];

    /// SQL literal values for primary key columns
    fn pkey_values(&self) -> Vec<String>;

    /// Query binding for only pkey columns
    fn pkey_bind<'q>(
        &'q self,
        query: Query<'q, DB, DB::Arguments<'q>>,
    ) -> Query<'q, DB, DB::Arguments<'q>>;

    /// List of non-primary-key column names
    fn data_columns() -> &'static [&'static str];

    /// SQL literal values for non-primary-key columns
    fn data_values(&self) -> Vec<String>;

    /// Query binding for only non-primary-key columns
    fn data_bind<'q>(
        &'q self,
        query: Query<'q, DB, DB::Arguments<'q>>,
    ) -> Query<'q, DB, DB::Arguments<'q>>;
}
