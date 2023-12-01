use log::info;

use super::PeeperClient;
use crate::app::domain::models::{
    drom_bull::DromBull, drom_bull_repository::DromBullRepository, user_request::UserRequest,
};
use async_trait::async_trait;

use scraper::{ElementRef, Html, Selector};

pub struct DromClient {
    pub base_uri: String,
}

impl DromClient {
    pub fn new() -> Self {
        Self {
            base_uri: std::env::var("DROM_BASE_URI").expect("DROM_BASE_URI must be set"),
        }
    }

    fn get_url(&mut self, request: &UserRequest, page: u32) -> reqwest::Url {
        let mut url = self.base_uri.clone();
        url.push_str(&request.firm);
        url.push_str("/");
        url.push_str(&request.model);
        url.push_str("/page");
        url.push_str(&page.to_string());
        url.push_str("/");

        let params = [
            ("minyear", 2010.to_string()),
            // ("maxyear", 2024.to_string()),
            // ("fueltype", 1.to_string()),
            // ("privod", 1.to_string()),
        ];
        print!(" 1{:?} ", url);
        reqwest::Url::parse_with_params(&url, params.clone()).unwrap()
    }

    pub async fn async_search(
        &mut self,
        request: &UserRequest,
    ) -> Result<(), Box<dyn std::error::Error>> {
        info!("Начало загрузки {:?} {:?}", &request.firm, &request.model);
        let mut next_page = true;
        let mut current_page = 1;

        while next_page {
            next_page = false;

            let url_p = self.get_url(request, current_page);
            info!("url {:?}", url_p.clone().to_string());

            let response = reqwest::get(url_p.clone())
                .await
                .unwrap()
                .text()
                .await;

            let document = Html::parse_document(&response.unwrap());
            let main_block_selector = Selector::parse("a").unwrap(); // данные обёрнуты в ссылку
            let bull_candidates = document.select(&main_block_selector);

            for candidate in bull_candidates {
                let attr = candidate.attr("data-ftid");
                let bull_data = match attr {
                    Some(v) => v,
                    None => "",
                };

                // следующая страница
                if bull_data.eq("component_pagination-item-next") {
                    next_page = true;
                    current_page += 1;

                    if !std::env::var("PRODUCTION")
                        .unwrap_or("false".to_string())
                        .eq("true")
                    {
                        if current_page > 3 {
                            next_page = false;
                        }
                    }
                }

                if !bull_data.eq("bulls-list_bull") {
                    continue;
                }

                let mut drom_bull = DromBull::new();

                let href = candidate.attr("href").unwrap();
                let href_vec: Vec<&str> = href.split("/").collect();
                let href_end: Vec<&str> = href_vec.last().unwrap().split(".").collect();

                drom_bull.external_id = href_end[0].to_string();

                // название, моедель, год
                let title_selector = Selector::parse("span").unwrap();
                let title_candidates = candidate.select(&title_selector);

                for title_candidate in title_candidates {
                    let title_attr = title_candidate.attr("data-ftid");
                    let title_data = match title_attr {
                        Some(v) => v,
                        None => "",
                    };

                    if !title_data.eq("bull_title") {
                        continue;
                    }

                    let html = title_candidate.inner_html().clone();
                    let title_data_vec: Vec<&str> = html.split(",").collect();
                    let firm_and_model: Vec<&str> = title_data_vec[0].split(" ").collect();
                    let firm = firm_and_model[0];
                    let model = firm_and_model[1];
                    let year = title_data_vec[1].trim();

                    print!(" 1{:?} 2{:?} 3{:?}", firm, model, year);

                    drom_bull.firm = firm.to_string();
                    drom_bull.model = model.to_string();
                    drom_bull.year = year.parse::<u32>().unwrap();
                }

                // комплектация
                let complectation_selector = Selector::parse("div.css-o2r31p").unwrap();
                let complectation_candidates = candidate.select(&complectation_selector);

                for complectation_candidate in complectation_candidates {
                    let html = complectation_candidate.inner_html().clone();
                    // let mut data_vec: Vec<&str> = html.split(" ").collect();

                    // if data_vec.len() > 2 {
                    //     data_vec.remove(0);
                    //     data_vec.remove(0);
                    // }
                    
                    // let complectation = data_vec.join(" ");
                    let complectation = html;
                    drom_bull.complectation = complectation;

                    print!(" complectation {:?} \n", drom_bull.complectation);
                }

                // детали
                let detail_selector = Selector::parse("div").unwrap();
                let detail_candidates = candidate.select(&detail_selector);

                for detail_div_candidate in detail_candidates {
                    let detail_attr = detail_div_candidate.attr("data-ftid");
                    let details_div = match detail_attr {
                        Some(v) => v,
                        None => "",
                    };

                    if !details_div.eq("component_inline-bull-description") {
                        continue;
                    }

                    // объём и силы
                    let mut detail_inner_vec = detail_div_candidate
                        .descendants()
                        .filter_map(|child| ElementRef::wrap(child))
                        .flat_map(|el| el.text())
                        .filter(|&el| !el.eq(" ") && !el.eq(",") && !el.starts_with(".css"))
                        .collect::<Vec<_>>();

                    if detail_inner_vec.len() == 0 {
                        detail_inner_vec = vec![];
                    }

                    // пропуск, неполная информация
                    if detail_inner_vec.len() < 5 {
                        info!("пропуск, неполная информация, {:?}, {:?}", url_p.as_str(), drom_bull.external_id);
                        continue;
                    }

                    let motor_vec: Vec<&str> = detail_inner_vec[0].split(" л ").collect();
                    let motor_volume = match motor_vec.first() {
                        Some(m) => m,
                        None => "-",
                    };
                    let motor_power = match motor_vec.last() {
                        Some(m) => m.replace(" л.с.)", "").replace("(", "").replace(" ", ""),
                        None => "-".to_string(),
                    };

                    let mut fuel = "-";
                    let mut kpp = "-";
                    let mut privod = "-";
                    let mut probeg = "-".to_string();
                    if detail_inner_vec.len() > 1 {
                        fuel = detail_inner_vec[1];
                    }

                    if detail_inner_vec.len() > 2 {
                        kpp = detail_inner_vec[2];
                    }

                    if detail_inner_vec.len() > 3 {
                        privod = detail_inner_vec[3];
                    }

                    if detail_inner_vec.len() > 4 {
                        probeg = detail_inner_vec[4].replace(" ", "").replace("км", "");
                    }

                    drom_bull.motor_volume = (motor_volume.parse::<f32>().unwrap_or(0.0) * 10.0).round() / 10.0;
                    drom_bull.motor_power = motor_power.parse::<u32>().unwrap_or(0);
                    drom_bull.fuel = fuel.to_string();
                    drom_bull.kpp = kpp.to_string();
                    drom_bull.privod = privod.to_string();
                    drom_bull.probeg = probeg.parse::<u32>().unwrap_or(0);

                    print!(" motor_volume {:?} motor_power {:?} fuel {:?} kpp {:?} privod {:?} probeg {:?}\n", motor_volume, motor_power, fuel, kpp, privod, probeg);
                }

                // цена
                let price_selector = Selector::parse("span").unwrap();
                let price_candidates = candidate.select(&price_selector);

                for price_candidate in price_candidates {
                    let price_attr = price_candidate.attr("data-ftid");
                    let price_data = match price_attr {
                        Some(v) => v,
                        None => "",
                    };

                    if !price_data.eq("bull_price") {
                        continue;
                    }

                    let html = price_candidate.inner_html().clone();
                    let price = html.trim().replace("&nbsp;", "").replace("<!-- -->", "");

                    drom_bull.price = price.parse::<u32>().unwrap();

                    // print!(" price{:?} ", price);
                }

                // город
                let location_selector = Selector::parse("span").unwrap();
                let location_candidates = candidate.select(&location_selector);

                for location_candidate in location_candidates {
                    let location_attr = location_candidate.attr("data-ftid");
                    let location_data = match location_attr {
                        Some(v) => v,
                        None => "",
                    };

                    if !location_data.eq("bull_location") {
                        continue;
                    }

                    let html = location_candidate.inner_html().clone();
                    let location = html.trim();
                    drom_bull.location = location.to_string();

                    // print!(" location{:?} ", location);
                }

                // если в базе есть эта машина по той же цене, то не сохраняем
                let existed =
                    DromBullRepository::get_identical(&drom_bull.external_id, drom_bull.price);

                if existed.id == 0 {
                    DromBullRepository::save(&mut drom_bull);
                }
            }
        }

        info!(
            "Конец загрузки {:?} {:?}. Страниц загружено {:?}",
            &request.firm,
            &request.model,
            &current_page - 1
        );

        Ok(())
    }
}

#[async_trait]
impl PeeperClient for DromClient {
    async fn search(&mut self, request: &UserRequest) {
        let _ = self.async_search(request).await;
    }
}
