use actix_web::{web, HttpResponse};
use sqlx::PgPool;

use crate::crypto::{Crypto, TokenResponse};
use crate::errors::TalliiError;
use crate::services::auth::AuthenticatedUser;
use crate::services::TalliiResponse;

use super::db::{InviteCodesTable, UsersTable};
use super::models::{CreateInviteCode, LoginUser, NewUser, UserQuery};

/// Gets all invite codes
pub async fn get_invite_codes(pool: web::Data<PgPool>, user: AuthenticatedUser) -> TalliiResponse {
    // check if the user is me to make sure that no one can make invite codes
    if user.username != String::from("adamaho") {
        return Err(TalliiError::UNAUTHORIZED.default());
    }

    // execute the query
    let all_invite_codes = InviteCodesTable::get_all(&pool).await?;

    // respond with all of the invite codes
    Ok(HttpResponse::Ok().json(all_invite_codes))
}

/// Checks the validity of the invite code
pub async fn check_invite_code(pool: web::Data<PgPool>, code: web::Path<String>) -> TalliiResponse {
    // execute the query
    let is_valid = InviteCodesTable::is_valid(&pool, &code).await?;

    let user = UsersTable::get_by_invite_code(&pool, &code).await?;

    // if not valid return an error
    if !is_valid || user.is_some() {
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
    if user.username != String::from("adamaho") {
        return Err(TalliiError::UNAUTHORIZED.default());
    }

    // execute the query
    InviteCodesTable::create_many(&pool, new_codes.amount).await?;

    // response with created
    Ok(HttpResponse::Created().finish())
}

/// Checks if the username is taken
pub async fn check_username(
    pool: web::Data<PgPool>,
    username: web::Path<String>,
) -> TalliiResponse {
    // execute the query
    match UsersTable::get_by_username(&pool, &username).await? {
        Some(_) => Err(TalliiError::USERNAME_TAKEN.default()),
        None => Ok(HttpResponse::Ok().finish()),
    }
}

/// Checks if the email is taken
pub async fn check_email(pool: web::Data<PgPool>, email: web::Path<String>) -> TalliiResponse {
    // execute the query
    match UsersTable::get_by_email(&pool, &email).await? {
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
    let user = match UsersTable::get_by_email(&pool, &person.email).await? {
        Some(u) => u,
        None => {
            return Err(TalliiError::INVALID_LOGIN.default());
        }
    };

    // verify the provided password
    if !crypto
        .verify_password(&person.password, &user.password)
        .await?
    {
        return Err(TalliiError::INVALID_LOGIN.default());
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
    let is_valid_invite_code = InviteCodesTable::is_valid(&pool, &new_user.invite_code).await?;

    // if not valid return an InvalidInviteCode error
    if !is_valid_invite_code {
        return Err(TalliiError::INVALID_INVITE_CODE.default());
    }

    // check to make sure the invite code is not taken by another user
    if let Some(_) = UsersTable::get_by_invite_code(&pool, &new_user.invite_code).await? {
        return Err(TalliiError::INVALID_INVITE_CODE.default());
    }

    // create the new user in the database
    let created_user = UsersTable::create(&pool, &new_user, &crypto).await?;

    // TODO: send verification email to user

    // create a new jwt token for that user
    let token = crypto
        .generate_jwt(created_user.user_id, created_user.username)
        .await?;

    // respond with the newly created token
    Ok(HttpResponse::Ok().json(TokenResponse { token }))
}

/// Gets the profile of a specific user
pub async fn get_user(
    pool: web::Data<PgPool>,
    username: web::Path<String>,
    _user: AuthenticatedUser,
) -> TalliiResponse {
    // get me from the database
    let user = UsersTable::get_by_username(&pool, &username).await?;

    // response with json of me
    Ok(HttpResponse::Ok().json(user))
}

/// Gets the profile of me
pub async fn get_me(pool: web::Data<PgPool>, user: AuthenticatedUser) -> TalliiResponse {
    // get me from the database
    let me = UsersTable::get_by_username(&pool, &user.username).await?;

    // response with json of me
    Ok(HttpResponse::Ok().json(me))
}

/// Gets maximum 10 users that match the provided username
pub async fn search_users(
    pool: web::Data<PgPool>,
    _user: AuthenticatedUser,
    params: web::Query<UserQuery>,
) -> TalliiResponse {
    // search for users in db
    let users = UsersTable::search_by_username(&pool, &params).await?;

    // response with json of users
    Ok(HttpResponse::Ok().json(users))
}
