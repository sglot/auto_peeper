use std::collections::HashMap;

use crate::{
    app::domain::{
        clients::{
            get_client,
            peeper_client_enum::{PeeperClientType, PeeperClientTypeData},
        },
        models::progress::Progress,
        support, user_request_mapper,
    },
    REQUEST_CONFIG,
};

use super::models::{progress_repository::ProgressRepository, user_request::UserRequestBundle};

pub async fn load() {
    let request_bundle: UserRequestBundle = match REQUEST_CONFIG.location.as_str() {
        "file" => confy::load_path(&REQUEST_CONFIG.location_file_path).unwrap_or_default(),
        &_ => confy::load_path(&REQUEST_CONFIG.location_file_path).unwrap_or_default(),
    };

    println!("Found req bundle {:?}", request_bundle);

    // проверка прогресса обработки пользовательских запросов
    clear_progress(&request_bundle);

    // подготовка параметров
    let progress_map = ProgressRepository::all();

    let enable_drom = std::env::var("ENABLE_DROM")
        .unwrap_or("true".to_string())
        .parse::<bool>()
        .unwrap_or(true);

    let enable_avito = std::env::var("ENABLE_AVITO")
        .unwrap_or("true".to_string())
        .parse::<bool>()
        .unwrap_or(true);

    // загрузка

    // println!("Found req {:?}", user_request);

    if enable_drom {
        println!("load drom");
        load_drom(progress_map.clone(), request_bundle.clone());
    }

    if enable_avito {
        println!("load avito");
        load_avito(progress_map, request_bundle).await;
    }
}

fn load_drom(map: HashMap<String, Progress>, request_bundle: UserRequestBundle) {
    for user_request in request_bundle.requests.iter() {
        let progress = match map.get(&support::make_key(
            user_request.id.to_string(),
            &PeeperClientTypeData::new(PeeperClientType::Drom).name,
        )) {
            Some(p) => p.clone(),
            None => Progress::new(),
        };

        if progress.is_loaded {
            continue;
        }

        let mut client = get_client(PeeperClientType::Drom);
        let page = progress.page.clone();
        let req = user_request.clone();
        tokio::spawn(async move {
            client
                .search(&&user_request_mapper::to_drom(&req), &page)
                .await
        });
    }
}

async fn load_avito(map: HashMap<String, Progress>, request_bundle: UserRequestBundle) {
    for user_request in request_bundle.requests.iter() {
        let progress = match map.get(&support::make_key(
            user_request.id.to_string(),
            &PeeperClientTypeData::new(PeeperClientType::Avito).name,
        )) {
            Some(p) => p.clone(),
            None => Progress::new(),
        };

        if progress.is_loaded {
            continue;
        }

        let mut client = get_client(PeeperClientType::Avito);
        // tokio::spawn(async move {
            client
                .search(&&user_request_mapper::to_drom(&user_request), &progress.page)
                .await;
        // });
    }
}

fn clear_progress(request_bundle: &UserRequestBundle) {
    let mut count_map: HashMap<&str, usize> = HashMap::new();

    let d = PeeperClientTypeData::new(PeeperClientType::Drom);
    count_map.insert(&d.name, request_bundle.requests.len());

    let a = PeeperClientTypeData::new(PeeperClientType::Avito);
    count_map.insert(
        &a.name,
        request_bundle
            .requests
            .iter()
            .filter(|request| !request.avito_model_hash.eq("") && !request.avito_filter_hash.eq(""))
            .count(),
    );

    for system in count_map {
        ProgressRepository::delete_finished(system.1, system.0);
    }
}
