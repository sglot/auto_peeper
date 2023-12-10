// Поиск объявлений

// Точка входа: https://api.drom.ru/v1.2/bulls/search
// Метод запроса: GET

// Описание параметров:

//     firmId - Фирма автомобиля (int)
//     modelId - Модель автомобиля (int)
//     generationNumber - Номер поколения (int)
//     restylingNumber - Номер рестайлинга (int)
//     cityId - Город (int)
//     regionId - Регион (int)
//     minYear - Год выпуска автомобиля (нижняя граница) (int)
//     maxYear - Год выпуска автомобиля (верхняя граница) (int)
//     minPrice - Стоимость автомобиля (нижняя граница) (int)
//     maxPrice - Стоимость автомобиля (верхняя граница) (int)
//     minEngineVolume - Объем двигателя (нижняя граница) (float)
//     maxEngineVolume - Объем двигателя (верхняя граница) (float)
//     minEnginePower - Лошадиные силы (нижняя граница) (int)
//     maxEnginePower - Лошадиные силы (верхняя граница) (int)
//     minMileageKm - Пробег автомобиля (нижняя граница) (int)
//     maxMileageKm - Пробег автомобиля (верхняя граница) (int)
//     frameType - Тип кузова (int)
//     colorId - Цвет кузова (int)
//     transmissionType - Тип КПП (int)
//     driveType - Тип привода (int)
//     fuelType - Тип топлива (int)
//     wheel - Расположение руля (int)
//     distance - Дистанция от города (int)
//     locationType - Местоположение авто (в наличии, в пути, под заказ) (int)
//     isHybrid - Гибрид (bool)
//     isGboExists - Установленно ли ГБО (bool)
//     isDamaged - Состояние (bool)
//     isNew - Новое авто (bool)
//     ph - Объявление с фотографией (bool)
//     withPhoto - Объявление с фотографией (bool)
//     withoutDocuments - Наличие ПТС (bool)
//     notUsedInRussia - Без пробега по РФ (bool)
//     unsold - Непроданные (bool)
//     neighborhood - Ближайшие города (bool)
//     orderBy - Поле сортировки (string)
//     revertSort - Порядок сортировки: true (desc) - по убыванию; false (asc) - по возрастанию (bool)
//     page - Страница (int)

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct UserRequestBundle {
    pub requests: Vec<UserRequest>
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct UserRequest {
    pub id: u32,
    pub firm: String,
    pub model: String,
    pub avito_model_hash: String,
    pub avito_filter_hash: String,
}