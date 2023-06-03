CREATE TABLE primary_users (
  id BIGINT NOT NULL PRIMARY KEY
) CHARACTER SET utf8mb4 COLLATE utf8mb4_bin;

CREATE TABLE line_auth (
  line_id VARCHAR(36) NOT NULL PRIMARY KEY,
  primary_user_id BIGINT NOT NULL,
  FOREIGN KEY(primary_user_id) REFERENCES primary_users(id) NOT NULL,
  display_name VARCHAR(255) NOT NULL DEFAULT '',
  picture_url VARCHAR(255) NOT NULL DEFAULT '',
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
) CHARACTER SET utf8mb4 COLLATE utf8mb4_bin;
