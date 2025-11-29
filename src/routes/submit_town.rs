use askama::Template;
use askama_web::WebTemplate;
use axum::{Router, extract::State, routing::get};
use chrono::{Duration, NaiveDate, Utc};
use chrono_tz::America::New_York;
use serde::Deserialize;

use crate::{SharedState, domain::Town, extract::current_user::CurrentUser, routes::SharedContext};

pub fn routes() -> Router<SharedState> {
    Router::new().route("/submit-town", get(submit_town_form))
}

#[derive(Template, WebTemplate, Default)]
#[template(path = "submit_town.html")]
struct SubmitTownPage {
    shared: SharedContext,
    form: SubmitTownForm,
    towns: Vec<Town>,
    max_race_date: NaiveDate,
}

#[derive(Deserialize, Debug, Default)]
struct SubmitTownForm {
    town_id: Option<String>,
    race_id: Option<String>,
    distance_val: Option<String>,
    distance_unit: Option<String>,
    race_date: Option<NaiveDate>,
    notes: Option<String>,
}

async fn submit_town_form(
    State(state): State<SharedState>,
    CurrentUser(user): CurrentUser,
) -> SubmitTownPage {
    SubmitTownPage {
        shared: SharedContext::new(&state.app_info, Some(*user)),
        towns: state.town_service.find_all().await,
        max_race_date: Utc::now().with_timezone(&New_York).date_naive(),
        ..Default::default()
    }
}
