use chrono::NaiveDate;

use serverless_pipeline::queries::process_month;

#[tokio::main]
async fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let input_data = NaiveDate::parse_from_str(&args[1], "%Y-%m-%d").expect("incorrect input data");

    process_month(input_data).await.unwrap();
}
