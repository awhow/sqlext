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
    /// columns() = key_columns() + value_columns()
    fn columns() -> &'static [&'static str];

    /// Query binding for all columns
    fn bind<'q>(
        &'q self,
        query: Query<'q, DB, DB::Arguments<'q>>,
    ) -> Query<'q, DB, DB::Arguments<'q>>;

    /// List of primary key column names
    fn pkey_columns() -> &'static [&'static str];

    /// Query binding for only pkey columns
    fn pkey_bind<'q>(
        &'q self,
        query: Query<'q, DB, DB::Arguments<'q>>,
    ) -> Query<'q, DB, DB::Arguments<'q>>;

    /// List of non-primary-key columns names
    fn data_columns() -> &'static [&'static str];

    /// Query binding for only non-primary-key columns
    fn data_bind<'q>(
        &'q self,
        query: Query<'q, DB, DB::Arguments<'q>>,
    ) -> Query<'q, DB, DB::Arguments<'q>>;
}
