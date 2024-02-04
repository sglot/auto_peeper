# Описание
Приложение для сбора информации с автомобильных агрегаторов

# Установка
1. Скопировать файл .exe в желаемый каталог
2. Сформировать список из желаемых авто в файле src\storage\requests\requests.toml
3. Добавить в атозагрузку


# Сборка приложения
Build command `cargo rustc --release --package auto_peeper -- -Clink-args="/SUBSYSTEM:WINDOWS /ENTRY:mainCRTStartup"`

SELECT date, model, avg(price), min(price), max(price) FROM "cars" GROUP BY date, model
ORDER BY date DESC;


# Формат запросов
[[requests]]
id = 1
firm = "toyota"
model = "corolla"
avito_model_hash = "ASgBAgICAkTgtg20mSjitg2woig"
avito_filter_hash = "ASgBAgECAkTgtg20mSjitg2woigBRfqMFBd7ImZyb20iOjIwMTgsInRvIjpudWxsfQ"

[[requests]]
id = 2
firm = "lada"
model = "granta"
avito_model_hash = "" // запрос к авито с таким параметром выполнен не будет
avito_filter_hash = "" // запрос к авито с таким параметром выполнен не будет