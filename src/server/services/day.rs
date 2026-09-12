use crate::schema::day::{
    CreateDaySchema, DaySchema, DeleteDaySchema, FindDaysByUserSchema, UpdateDayNotesSchema,
    UpdateDayTargetCaloriesSchema, UpdateDayWeightSchema,
};
use crate::server::error::ServerError;
use crate::server::repo::day::{CreateDay, Day};
use sqlx::PgPool;

pub async fn list(user_name: &str, pool: &PgPool) -> Result<Vec<DaySchema>, ServerError> {
    let days = Day::find_by_user(
        &FindDaysByUserSchema {
            name: user_name.to_string(),
        },
        pool,
    )
    .await?;

    Ok(days.into_iter().map(DaySchema::from).collect())
}

pub async fn create(payload: &CreateDaySchema, pool: &PgPool) -> Result<DaySchema, ServerError> {
    let target_calories = match payload.target_calories {
        Some(target_calories) => target_calories,
        None => Day::find_previous(&payload.user_name, payload.date, pool)
            .await?
            .map(|previous| previous.target_calories)
            .ok_or(ServerError::MissingTargetCalories)?,
    };

    let day = Day::create(
        &CreateDay {
            date: payload.date,
            user_name: payload.user_name.clone(),
            weight_kg: payload.weight_kg,
            target_calories,
            notes: payload.notes.clone(),
        },
        pool,
    )
    .await?;

    Ok(day.into())
}

pub async fn update_weight(
    payload: UpdateDayWeightSchema,
    pool: &PgPool,
) -> Result<DaySchema, ServerError> {
    let day = Day::update_weight_kg(payload, pool).await?;
    Ok(day.into())
}

pub async fn update_target_calories(
    payload: UpdateDayTargetCaloriesSchema,
    pool: &PgPool,
) -> Result<DaySchema, ServerError> {
    let day = Day::update_target_calories(payload, pool).await?;
    Ok(day.into())
}

pub async fn update_notes(
    payload: UpdateDayNotesSchema,
    pool: &PgPool,
) -> Result<DaySchema, ServerError> {
    let day = Day::update_notes(payload, pool).await?;
    Ok(day.into())
}

pub async fn delete(payload: &DeleteDaySchema, pool: &PgPool) -> Result<(), ServerError> {
    Day::delete(payload, pool).await
}
