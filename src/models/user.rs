use password_auth::VerifyError;
use crate::error::AppError;
use crate::repository::Repository;

pub struct UnauthenticatedUser {
    username: String,
    password: String,
}

impl UnauthenticatedUser {
    pub fn new(username: String, password: String) -> Self {
        Self {
            username,
            password
        }
    }
    
    pub async fn authenticate(&self, repository: &Repository) -> Result<User, AppError> {
        let user_record = match repository
            .get_user_by_username(self.username.as_str()).await? {
            Some(user_record) => user_record,
            None => return Err(AppError::UserDoesNotExists)
        };

        match password_auth::verify_password(self.password.as_str(), &user_record.password_hash) {
            Ok(()) => Ok(User::new(user_record.id, user_record.username)),
            Err(VerifyError::PasswordInvalid) => Err(AppError::InvalidCredentials),
            Err(VerifyError::Parse(err)) => panic!("Hashing algorithm failed: {}", err)
        }
    }

    pub async fn register(&self, repository: Repository) -> Result<User, AppError> {
        let password_hash = password_auth::generate_hash(self.password.as_str());
        let user_record = match repository.add_user(&self.username, password_hash.as_str()).await {
            Ok(user_record) => user_record,
            Err(sqlx::Error::Database(db_err)) 
                if db_err.is_unique_violation() => {
                return Err(AppError::UsernameTaken)
            },
            Err(err) => return Err(AppError::Database(err))
        };
        Ok(User::new(user_record.id, user_record.username))

    }
}

pub struct User {
    id: i64,
    username: String,
}

impl User {
    fn new(id: i64, username: String) -> Self {
        Self { id, username }
    }
    pub fn username(&self) -> &str {
        &self.username
    }
    
    pub fn id(&self) -> i64 {
        self.id
    }
}

#[derive(sqlx::FromRow)]
pub struct UserRecord {
    pub id: i64,
    pub username: String,
    pub password_hash: String
}