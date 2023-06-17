CREATE TABLE primary_users (
  id BIGINT AUTO_INCREMENT NOT NULL PRIMARY KEY,
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
) CHARACTER SET utf8mb4;

CREATE TABLE line_auth (
  line_id VARCHAR(32) NOT NULL PRIMARY KEY, -- ユーザーIDの値は、U[0-9a-f]{32}の正規表現にマッチする文字列
  primary_user_id BIGINT NOT NULL,
  FOREIGN KEY(primary_user_id) REFERENCES primary_users(id) NOT NULL,
  display_name VARCHAR(255) NOT NULL DEFAULT '', -- displayNameの制限の記述は見つけられなかった（https://developers.line.biz/ja/reference/messaging-api/#get-profile）
  picture_url VARCHAR(2048) NOT NULL DEFAULT '', -- 一般的なURL長内に収まる想定
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
) CHARACTER SET utf8mb4 COLLATE utf8mb4_bin;

CREATE TABLE talk_rooms (
  id BIGINT AUTO_INCREMENT NOT NULL PRIMARY KEY,
  document_id VARCHAR(36) NOT NULL UNIQUE, -- uuid v4を使っても36桁
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
) CHARACTER SET utf8mb4;
