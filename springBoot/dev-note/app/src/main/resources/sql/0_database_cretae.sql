#데이터 베이스생성
CREATE DATABASE dev_note CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

#계정 생성
CREATE USER 'dev_note'@'localhost' IDENTIFIED BY 'dev_note1234';

GRANT ALL PRIVILEGES ON dev_note.* TO 'dev_note'@'localhost';

FLUSH PRIVILEGES;