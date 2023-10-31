CREATE TABLE primary_users (
  id BIGINT AUTO_INCREMENT NOT NULL PRIMARY KEY,
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
) CHARACTER SET utf8mb4;

-- line_id: ユーザーIDの値は、U[0-9a-f]{32}の正規表現にマッチする文字列
-- display_name: displayNameの制限の記述は見つけられなかった（https://developers.line.biz/ja/reference/messaging-api/#get-profile）
-- picture_url: 一般的なURL長内に収まる想定
CREATE TABLE line_users (
  line_id VARCHAR(32) NOT NULL PRIMARY KEY,
  primary_user_id BIGINT NOT NULL,
  FOREIGN KEY(primary_user_id) REFERENCES primary_users(id),
  display_name VARCHAR(255) NOT NULL DEFAULT '',
  picture_url VARCHAR(2048) NOT NULL DEFAULT '',
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
) CHARACTER SET utf8mb4 COLLATE utf8mb4_bin;

CREATE INDEX idx_line_users_primary_user_id ON line_users(primary_user_id);
