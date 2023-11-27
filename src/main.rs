mod days;
use days::day_0;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let day0 = day_0::routes();
    let router = day0;

    Ok(router.into())
}
