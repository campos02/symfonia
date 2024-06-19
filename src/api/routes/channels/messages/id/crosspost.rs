use chorus::types::{jwt::Claims, MessageSendSchema, Snowflake};
use poem::{
    handler,
    IntoResponse,
    web::{Data, Json, Path},
};
use sqlx::MySqlPool;

use crate::{
    database::entities::{Channel, Message, User},
    errors::{ChannelError, Error},
};

#[handler]
pub async fn create_crosspost_message(
    Data(db): Data<&MySqlPool>,
    Data(_claims): Data<&Claims>,
    Data(authed_user): Data<&User>,
    Path(channel_id): Path<Snowflake>,
    Json(payload): Json<MessageSendSchema>,
) -> poem::Result<impl IntoResponse> {
    let channel = Channel::get_by_id(db, channel_id)
        .await?
        .ok_or(Error::Channel(ChannelError::InvalidChannel))?;

    let Some(referenced) = &payload.message_reference else {
        return Err(Error::Channel(ChannelError::InvalidMessage).into()); // TODO: Maybe a generic bad request error?
    };

    let referenced_message = Message::get_by_id(db, referenced.channel_id, referenced.message_id)
        .await?
        .ok_or(Error::Channel(ChannelError::InvalidMessage))?;

    let message = Message::create(
        db,
        payload,
        channel.guild_id,
        referenced_message.channel_id,
        authed_user.id,
    )
    .await?;

    Ok(Json(message))
}
