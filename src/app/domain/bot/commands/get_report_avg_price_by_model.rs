use crate::app::domain::{models::reports::report_avg_price_by_model_repository::ReportAVGPriceByModelRepository, support::separated_string};

pub fn get_report_avg_price_by_model(model: &str) -> String {
    let cars = ReportAVGPriceByModelRepository::get(model);
    
    let mut text = format!("{} \n \n", model);
    for car in cars {
        text.push_str(format!("{}.....{} р. \n", car.month, separated_string(car.avg.round())).as_str());
    }

    text
}