use diesel::{r2d2::{ConnectionManager,Pool, PooledConnection}, PgConnection};

fn get_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("failed to build pool")
}

pub fn establish_connection() -> PooledConnection<ConnectionManager<PgConnection>> {
    let pool = get_connection_pool();
    pool.get().unwrap()
}
