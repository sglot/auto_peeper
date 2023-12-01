# Описание
Приложение для сбора информации с автомобильных агрегаторов

# Установка
1. Скопировать файл .exe в желаемый каталог
2. Сформировать список из желаемых авто в файле src\storage\requests\requests.toml
3. Добавить в атозагрузку


# Сборка приложения
Build command `cargo rustc --release --package auto_peeper -- -Clink-args="/SUBSYSTEM:WINDOWS /ENTRY:mainCRTStartup"`
