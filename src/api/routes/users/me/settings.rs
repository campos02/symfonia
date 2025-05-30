/*
 *  This Source Code Form is subject to the terms of the Mozilla Public
 *  License, v. 2.0. If a copy of the MPL was not distributed with this
 *  file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::{
    database::entities::User,
    errors::{Error, UserError},
};
use chorus::types::{jwt::Claims, UserSettings};
use poem::{
    handler,
    web::{Data, Json},
    IntoResponse,
};
use sqlx::PgPool;

#[handler]
pub async fn get_settings(
    Data(db): Data<&PgPool>,
    Data(claims): Data<&Claims>,
) -> poem::Result<impl IntoResponse> {
    let user = User::get_by_id(db, claims.id)
        .await?
        .ok_or(Error::User(UserError::InvalidUser))?;

    Ok(Json(user.settings))
}

#[handler]
pub async fn update_settings(
    Data(db): Data<&PgPool>,
    Data(claims): Data<&Claims>,
    Json(settings): Json<UserSettings>,
) -> poem::Result<impl IntoResponse> {
    let mut user = User::get_by_id(db, claims.id)
        .await?
        .ok_or(Error::User(UserError::InvalidUser))?;

    user.settings =
        crate::database::entities::UserSettings::consume(settings, user.settings_index.to_uint());
    // TODO: user.settings.update(db).await.map_err(Error::SQLX)?;

    Ok(Json(user.settings))
}
