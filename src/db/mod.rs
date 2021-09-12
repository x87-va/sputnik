use async_trait::async_trait;
use deadpool::managed::{Manager, RecycleResult};
use sqlx::{Connection, Error as SqlxError, PgConnection};

// TODO:
// pub struct Pool {
//   pool: deadpool::managed::Pool<PoolManager>
// }

// impl Pool {

//   pub fn new(url: String, size: usize) -> Pool {
//     let manager = PoolManager {
//       url
//     };

//     let pool = Pool {
//       pool: deadpool::managed::Pool::new(manager, size)      
//     };

//     return pool;
//   }

// 	pub async fn get(&self) -> Result<PgConnection, SqlxError> {
// 		self.pool.get().await
// 	}
// }

pub type Pool = deadpool::managed::Pool<PoolManager>;

pub struct PoolManager {
  pub url: String,
}

#[async_trait]
impl Manager for PoolManager {
	type Type = PgConnection;
	type Error = SqlxError;

	async fn create(&self) -> Result<PgConnection, SqlxError> {
		PgConnection::connect(&self.url).await
	}

	async fn recycle(&self, obj: &mut PgConnection) -> RecycleResult<SqlxError> {
		Ok(obj.ping().await?)
	}

	fn detach(&self, _obj: &mut Self::Type) {}
}
