use actix_web::{web, HttpResponse};
use sqlx::PgPool;

use crate::crypto::{Crypto, TokenResponse};
use crate::errors::TalliiError;
use crate::services::auth::AuthenticatedUser;
use crate::services::TalliiResponse;

use super::db::{InviteCodeRepository, UserRepository};
use super::models::{CheckEmail, CheckUsername, CreateInviteCode, InviteCode, LoginUser, NewUser};

/// Gets all invite codes
pub async fn get_all_invite_codes(pool: web::Data<PgPool>) -> TalliiResponse {
    // execute the query
    let all_invite_codes = InviteCodeRepository::get_all(&pool).await?;

    // respond with all of the invite codes
    Ok(HttpResponse::Ok().json(all_invite_codes))
}

/// Checks the validity of the invite code
pub async fn check_invite_code(
    pool: web::Data<PgPool>,
    code: web::Json<InviteCode>,
) -> TalliiResponse {
    // execute the query
    let is_valid = InviteCodeRepository::is_valid(&pool, &code.id).await?;

    // if not valid return an error
    if !is_valid {
        Err(TalliiError::INVALID_INVITE_CODE.default())
    } else {
        Ok(HttpResponse::Ok().json(""))
    }
}

/// Creates new invite codes
pub async fn create_invite_codes(
    pool: web::Data<PgPool>,
    new_codes: web::Json<CreateInviteCode>,
    user: AuthenticatedUser,
) -> TalliiResponse {
    // check if the user is me to make sure that no one can make invite codes
    if user.username != String::from("aho") {
        return Err(TalliiError::UNAUTHORIZED.default());
    }

    // execute the query
    InviteCodeRepository::create_many(&pool, new_codes.amount).await?;

    // response with created
    Ok(HttpResponse::Created().finish())
}

/// Checks if the username is taken
pub async fn check_username(
    pool: web::Data<PgPool>,
    payload: web::Json<CheckUsername>,
) -> TalliiResponse {
    // execute the query
    match UserRepository::get_by_username(&pool, &payload.username).await? {
        Some(_) => Err(TalliiError::USERNAME_TAKEN.default()),
        None => Ok(HttpResponse::Ok().finish()),
    }
}

/// Checks if the email is taken
pub async fn check_email(
    pool: web::Data<PgPool>,
    payload: web::Json<CheckEmail>,
) -> TalliiResponse {
    // execute the query
    match UserRepository::get_by_email(&pool, &payload.email).await? {
        Some(_) => Err(TalliiError::EMAIL_TAKEN.default()),
        None => Ok(HttpResponse::Ok().finish()),
    }
}

/// Logs the user in if the provided credentials are correct
pub async fn login(
    pool: web::Data<PgPool>,
    crypto: web::Data<Crypto>,
    person: web::Json<LoginUser>,
) -> Result<HttpResponse, TalliiError> {
    // check if there is a user with the provided email
    let user = match UserRepository::get_by_email(&pool, &person.email).await? {
        Some(u) => u,
        None => {
            return Err(TalliiError::UNAUTHORIZED.default());
        }
    };

    // verify the provided password
    if !crypto
        .verify_password(&person.password, &user.password)
        .await?
    {
        return Err(TalliiError::UNAUTHORIZED.default());
    }

    // create a new jwt for the newly authorized user
    let token = crypto.generate_jwt(user.user_id, user.username).await?;

    // respond with the token
    Ok(HttpResponse::Ok().json(TokenResponse { token }))
}

/// Signs a user up with the provided credentials
pub async fn signup(
    pool: web::Data<PgPool>,
    crypto: web::Data<Crypto>,
    new_user: web::Json<NewUser>,
) -> TalliiResponse {
    // check to make sure the invite code is valid
    let is_valid_invite_code = InviteCodeRepository::is_valid(&pool, &new_user.invite_code).await?;

    // if not valid return an InvalidInviteCode error
    if !is_valid_invite_code {
        return Err(TalliiError::INVALID_INVITE_CODE.default());
    }

    // check to make sure the invite code is not taken by another user
    if let Some(_) = UserRepository::get_by_invite_code(&pool, &new_user.invite_code).await? {
        return Err(TalliiError::INVALID_INVITE_CODE.default());
    }

    // create the new user in the database
    let created_user = UserRepository::create(&pool, &new_user, &crypto).await?;

    // TODO: send verification email to user

    // create a new jwt token for that user
    let token = crypto
        .generate_jwt(created_user.user_id, created_user.username)
        .await?;

    // respond with the newly created token
    Ok(HttpResponse::Ok().json(TokenResponse { token }))
}
