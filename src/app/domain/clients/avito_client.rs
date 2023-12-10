use std::{ time::{Instant, Duration}, thread::sleep};

use log::info;
use reqwest::Client;

use super::PeeperClient;
use crate::{app::domain::{models::{
        drom_bull::DromBull, drom_bull_repository::DromBullRepository, user_request::UserRequest, progress::Progress, progress_repository::ProgressRepository, context_repository::ContextRepository,
    }, user_request_mapper}, REQUEST_CONFIG};
use async_trait::async_trait;

use scraper::{Html, Selector};

pub struct AvitoClient {
    pub base_uri: String,
    pub system_type: String,
    pub request_sleep: u64,
}

impl AvitoClient {
    pub fn new() -> Self {
        Self {
            base_uri: std::env::var("AVITO_BASE_URI").expect("AVITO_BASE_URI must be set"),
            system_type: "avito".to_string(),
            request_sleep: REQUEST_CONFIG.avito_sleep_seconds,
        }
    }

    fn get_url(&mut self, request: &UserRequest, page: u32) -> reqwest::Url {
        let mut url = self.base_uri.clone();
        url.push_str(&request.firm);
        url.push_str("/");
        url.push_str(&request.model);
        url.push_str("-");
        url.push_str(&request.avito_model_hash);

        let params = [("f", &request.avito_filter_hash), ("p", &page.to_string())];
        print!("url: {:?} \n", url);
        reqwest::Url::parse_with_params(&url, params.clone()).unwrap()
    }

    pub async fn async_search(
        &mut self,
        request: &UserRequest,
        start_page: &u32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        info!(
            "Начало загрузки [avito] {:?} {:?}",
            &request.firm, &request.model
        );
        let mut next_page = true;
        let mut current_page = *start_page;

        let client = Client::builder().cookie_store(true).build()?;
        let mut next_time = Instant::now() + Duration::from_secs(self.request_sleep);

        let mut progress = Progress::new();
        progress.request_id = request.id;
        progress.system = "avito".to_string();
        

        'page_loop: while next_page {
            current_page += 1;
            progress.page = current_page;
            next_page = false;

            // проверка пользовательских требований
            if request.avito_model_hash.eq("") || request.avito_filter_hash.eq("") {
                info!("[avito] {:?} {:?} Пустые параметры. Пропуск", &request.firm, &request.model);
                continue;
            }

            // формирование запроса
            let url_p = self.get_url(request, current_page);
            info!("url {:?} \n", url_p.clone().to_string());

            let response = client.get(url_p.clone()).send().await.unwrap().text().await;

            // промежуточное ожидание
            sleep(next_time - Instant::now());
            next_time += Duration::from_secs(self.request_sleep);


            // поиск данных по документу
            let document = Html::parse_document(&response.unwrap());
            let main_block_selector = Selector::parse("div").unwrap();
            let bull_candidates = document.select(&main_block_selector);

            'car_loop: for candidate in bull_candidates {
                let attr = candidate.attr("data-marker");
                let item_data = match attr {
                    Some(v) => v,
                    None => "",
                };

                // вылезла капча
                let captcha_div = candidate.attr("class");
                let captcha_div_class = match captcha_div {
                    Some(v) => v,
                    None => "",
                };
                if captcha_div_class.eq("firewall-container") {
                    info!("### ### ###\n ### ### Капча ### ### \n ### ### ###");

                    continue 'page_loop;
                }

                if !item_data.eq("item") {
                    continue 'car_loop;
                }

                let mut drom_bull = DromBull::new();
                drom_bull.system = self.system_type.clone();

                let id: &str = candidate.attr("id").unwrap();

                drom_bull.external_id = id.to_string();

                // название, модель, год
                let title_selector = Selector::parse("h3").unwrap();
                let title_candidates = candidate.select(&title_selector);

                for title_candidate in title_candidates {
                    let title_attr = title_candidate.attr("itemprop");
                    let title_data = match title_attr {
                        Some(v) => v,
                        None => "",
                    };

                    if !title_data.eq("name") {
                        continue;
                    }

                    let html = title_candidate.inner_html().clone();
                    let title_data_vec: Vec<&str> = html.split(",").collect();
                    let mut firm_and_model: Vec<&str> = title_data_vec[0].split(" ").collect();
                    // в конце указан объём двигателя и коробка
                    firm_and_model.remove(firm_and_model.len() - 1);
                    firm_and_model.remove(firm_and_model.len() - 1);

                    let firm = firm_and_model[0];

                    // сработает, если название марки из 1 слова
                    firm_and_model.remove(0);

                    let year = title_data_vec[1]
                        .to_string()
                        .replace(" ", "")
                        .replace("&nbsp;", "");

                    // print!(" 1{:?} 2{:?} 3{:?}", firm, model, year);

                    drom_bull.firm = firm.to_string();
                    drom_bull.model = user_request_mapper::model_from_avito(&request.model); // приводится к единому названию
                    drom_bull.year = year.parse::<u32>().unwrap_or(0);
                }

                // детали
                let detail_selector = Selector::parse("p").unwrap();
                let detail_candidates = candidate.select(&detail_selector);

                for detail_div_candidate in detail_candidates {
                    let detail_attr = detail_div_candidate.attr("data-marker");
                    let details_div = match detail_attr {
                        Some(v) => v,
                        None => "",
                    };

                    if !details_div.eq("item-specific-params") {
                        continue;
                    }

                    // объём и силы
                    let html = detail_div_candidate.inner_html();
                    let mut detail_inner_vec: Vec<&str> = html.split(", ").collect();

                    if detail_inner_vec.len() == 0 {
                        detail_inner_vec = vec![];
                    }

                    // пропуск, неполная информация
                    if detail_inner_vec.len() < 5 {
                        info!(
                            "пропуск, неполная информация, {:?}, {:?}",
                            url_p.as_str(),
                            drom_bull.external_id
                        );
                        continue;
                    }

                    let motor_vec: Vec<&str> = detail_inner_vec[1].split(" ").collect();
                    // пропуск, неполная информация
                    if motor_vec.len() < 3 {
                        info!(
                            "пропуск, неполная информация по объёму, коробке, силам, {:?}, {:?}",
                            url_p.as_str(),
                            drom_bull.external_id
                        );
                        continue;
                    }

                    let motor_volume = motor_vec[0];

                    let kpp = motor_vec[1];

                    let motor_power = match motor_vec.last() {
                        Some(m) => m
                            .replace("&nbsp;", "")
                            .replace("л.с.)", "")
                            .replace("(", "")
                            .replace(" ", ""),
                        None => "-".to_string(),
                    };

                    let probeg = detail_inner_vec[0]
                        .replace(" ", "")
                        .replace("км", "")
                        .replace("&nbsp;", "");

                    let privod = detail_inner_vec[3];

                    let fuel = detail_inner_vec[4];

                    drom_bull.motor_volume =
                        (motor_volume.parse::<f32>().unwrap_or(0.0) * 10.0).round() / 10.0;
                    drom_bull.motor_power = motor_power.parse::<u32>().unwrap_or(0);
                    drom_bull.fuel = fuel.to_string();
                    drom_bull.kpp = kpp.to_string();
                    drom_bull.privod = privod.to_string();
                    drom_bull.probeg = probeg.parse::<u32>().unwrap_or(0);

                    print!(" motor_volume {:?} motor_power {:?} fuel {:?} kpp {:?} privod {:?} probeg {:?}\n", motor_volume, motor_power, fuel, kpp, privod, probeg);
                }

                // цена
                let price_selector = Selector::parse("p>meta").unwrap();
                let price_candidates = candidate.select(&price_selector);

                for price_candidate in price_candidates {
                    let price_attr = price_candidate.attr("itemprop");
                    let price_data = match price_attr {
                        Some(v) => v,
                        None => "",
                    };

                    if !price_data.eq("price") {
                        continue;
                    }

                    let price = price_candidate.attr("content").unwrap();

                    drom_bull.price = price.parse::<u32>().unwrap_or(0);
                }

                // город
                let location_selector = Selector::parse("div.geo-root-zPwRk>p>span").unwrap();
                let location_candidates = candidate.select(&location_selector);

                for location_candidate in location_candidates {
                    // должен быть единственный элемент

                    let html = location_candidate.inner_html().clone();
                    let location = html.trim();
                    drom_bull.location = location.to_string();
                }

                // если в базе есть эта машина по той же цене, то не сохраняем
                let existed = DromBullRepository::get_identical(
                    &drom_bull.external_id,
                    drom_bull.price,
                    &drom_bull.system,
                );

                if existed.id == 0 {
                    print!("[avito] want to save {:?} ", drom_bull);
                    DromBullRepository::save(&mut drom_bull);
                }
            }

            // следующая страница
            let next_page_selector = Selector::parse("a").unwrap(); // данные обёрнуты в ссылку
            let next_page_candidates = document.select(&next_page_selector);

            for candidate in next_page_candidates {
                let attr = candidate.attr("data-marker");
                let item_data = match attr {
                    Some(v) => v,
                    None => "",
                };

                // проверка следующей страницы
                if item_data.eq("pagination-button/nextPage") {
                    next_page = true;

                    if !std::env::var("PRODUCTION")
                        .unwrap_or("false".to_string())
                        .eq("true")
                    {
                        if current_page > 20 {
                            next_page = false;
                        }
                    }

                    break;
                }
            }

            if !next_page {
                // все страницы загружены
                progress.is_loaded = true
            }

            //запись успешной попытки
            print!("[avito] want to save progress{:?} ", progress);
            ProgressRepository::upsert(&mut progress);
        }

        info!(
            "Конец загрузки [avito] {:?} {:?}. Страниц загружено {:?}",
            &request.firm,
            &request.model,
            &current_page - 1
        );

        ContextRepository::set_next_round_avito(true);

        Ok(())
    }
}

#[async_trait]
impl PeeperClient for AvitoClient {
    async fn search(&mut self, request: &UserRequest, start_page: &u32) {
        let _ = self.async_search(request, start_page).await;
    }
}
