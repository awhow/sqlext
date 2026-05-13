use sqlx::database::Database;
use sqlx::query::Query;

pub trait ToRow<DB>
where
    DB: Database,
{
    fn table_name() -> &'static str;

    fn columns() -> &'static [&'static str];

    fn bind<'q>(
        &'q self,
        query: Query<'q, DB, DB::Arguments<'q>>,
    ) -> Query<'q, DB, DB::Arguments<'q>>;
}
