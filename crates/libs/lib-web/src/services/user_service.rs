use chrono::NaiveDateTime;
use lib_core::db::Db;
use lib_core::model::role::RoleEnum;
use lib_core::model::user::{UserForCreate, UserForSelect, UserForUpdate, UserRepo};
use serde::Serialize;
use uuid::Uuid;

use crate::error::{Error, Result};

#[derive(Serialize, Clone)]
pub struct UserDto {
    pub id: Uuid,
    pub nickname: String,
    pub role: RoleEnum,
    pub email: String,
    #[serde(skip)]
    pub hashed_password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl UserDto {
    pub fn from_user(user: UserRepo) -> Self {
        Self {
            id: user.id,
            nickname: user.nickname,
            role: user.role,
            email: user.email,
            created_at: user.created_at,
            updated_at: user.updated_at,
            hashed_password: user.hashed_password,
        }
    }
}

/// Обертка для удобной работы с пользователями
pub struct UserService;

impl UserService {
    pub async fn get_meny_by_query(
        db: &Db,
        _requester_id: Option<Uuid>,
        query: &str,
    ) -> Result<Vec<UserDto>> {
        let users = UserRepo::find_many_by_query(db, query).await?;
        Ok(users.into_iter().map(UserDto::from_user).collect())
    }

    /// Создание нового пользователя
    pub async fn create(
        db: &Db,
        _requester_id: Option<Uuid>,
        nickname: &str,
        email: &str,
        hashed_password: &str,
    ) -> Result<UserDto> {
        let user = UserRepo::create(
            db,
            UserForCreate {
                nickname: nickname.to_string(),
                email: email.to_string(),
                hashed_password: hashed_password.to_string(),
            },
        )
        .await?;

        Ok(UserDto::from_user(user))
    }

    /// Получение пользователя по ID
    pub async fn get_by_id(db: &Db, _requester_id: Option<Uuid>, id: &Uuid) -> Result<UserDto> {
        let user = UserRepo::find(
            db,
            UserForSelect {
                id: Some(*id),
                ..Default::default()
            },
        )
        .await?;

        Ok(UserDto::from_user(user))
    }

    /// Получение пользователя по email
    pub async fn get_by_email(
        db: &Db,
        _requester_id: Option<Uuid>,
        email: &str,
    ) -> Result<UserDto> {
        let user = UserRepo::find(
            db,
            UserForSelect {
                email: Some(email.to_string()),
                ..Default::default()
            },
        )
        .await?;

        Ok(UserDto::from_user(user))
    }

    /// Получение пользователя по никнейму
    pub async fn get_by_nickname(
        db: &Db,
        _requester_id: Option<Uuid>,
        nickname: &str,
    ) -> Result<UserDto> {
        let user = UserRepo::find(
            db,
            UserForSelect {
                nickname: Some(nickname.to_string()),
                ..Default::default()
            },
        )
        .await?;

        Ok(UserDto::from_user(user))
    }

    /// Обновление данных пользователя
    pub async fn update(
        db: &Db,
        _requester_id: Option<Uuid>,
        id: &Uuid,
        nickname: Option<String>,
        email: Option<String>,
        role: Option<RoleEnum>,
        hashed_password: Option<String>,
    ) -> Result<UserDto> {
        let user = UserRepo::update(
            db,
            id,
            UserForUpdate {
                nickname,
                email,
                role,
                hashed_password,
            },
        )
        .await?;

        Ok(UserDto::from_user(user))
    }

    /// Удаление пользователя
    pub async fn delete(db: &Db, _requester_id: Option<Uuid>, id: &Uuid) -> Result<()> {
        UserRepo::delete(db, &id).await.map_err(Error::Core)
    }

    /// Получение списка всех пользователей
    pub async fn get_all(db: &Db, _requester_id: Option<Uuid>) -> Result<Vec<UserDto>> {
        let users = UserRepo::find_all(db, UserForSelect::default()).await?;
        Ok(users.into_iter().map(UserDto::from_user).collect())
    }

    /// Получение пользователей по роли
    pub async fn get_by_role(
        db: &Db,
        _requester_id: Option<Uuid>,
        role: RoleEnum,
    ) -> Result<Vec<UserDto>> {
        let users = UserRepo::find_all(
            db,
            UserForSelect {
                role: Some(role),
                ..Default::default()
            },
        )
        .await?;

        Ok(users.into_iter().map(UserDto::from_user).collect())
    }
}
