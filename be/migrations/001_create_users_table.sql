-- 创建用户表
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL,
    age INTEGER,
    gender SMALLINT NOT NULL DEFAULT 1,  -- 1: Male, 2: Female
    email VARCHAR(255),
    is_deleted BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- 创建索引
CREATE INDEX IF NOT EXISTS idx_users_username ON users(username);
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);

-- 插入测试数据（可选）
INSERT INTO users (username, password, age, gender, email, is_deleted)
VALUES ('testuser', '$2b$12$K2/xFxH/xZ9Yx7x/xZ9Yx', 25, 1, 'test@example.com', FALSE)
ON CONFLICT (username) DO NOTHING;
