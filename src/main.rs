mod days;
use days::{day_0, day_1};

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let day0 = day_0::routes();
    let day1 = day_1::routes();
    let router = day0.merge(day1);

    Ok(router.into())
}
