use axum::{
    Json,
    extract::State,
    extract::Query,
};

use serde::{Serialize, Deserialize};
use sqlx::postgres::PgPool;


//Serialize Rust->JSON
//Deserialize JSON->Rust struct
#[derive(Serialize, Deserialize)]
pub struct Prediction{
    product_id: String,
    value: f64,
}


#[derive(Deserialize)]
pub struct PredictionParams{
    product_id:String,
}

pub async fn get_predictions(State(db): State<PgPool>, Query(params):Query<PredictionParams>,) -> Json<Prediction>{
    //query_as! performs compile time SQL verification
    let prediction = sqlx::query_as!(
        Prediction,
        "SELECT product_id, value FROM public prediction WHERE product_id = $1",
        params.product_id,
    )
    //awaits for exactly one row will throw err if not given 1
    .fetch_one(&db)
    .await.unwrap()?;

    Json(prediction)
}
