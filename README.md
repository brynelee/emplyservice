# emplyservice
employee service demo by Rust

# create the project
cargo new emplyservice


# configure Cargo.toml
[dependencies]
diesel = { version = "2.0.0", features = ["postgres"] }
dotenvy = "0.15"
​
# install diesel CLI
# needed to install postgres client lib in the OS env
cargo install diesel_cli --no-default-features --features postgres
​
# configure .env
# xdpostgres is the service IP address, my usage is to setup /etc/host to map that to docker service or k8s service name
# employeesrest_api is the schema name of the database
DATABASE_URL=postgres://postgres:p0stgres@xdpostgres/employeesrest_api
​
# diesel 创建数据库, 创建空的migration目录用来管理schema，有up.sql & down.sql
# 创建diesel.toml文件，设置schema.rs，设置migration目录配置信息
diesel setup
​
# 这个期间如果报libxxx.dll找不到，就从postgresql的bin目录下复制文件到lib目录下即可
​
# up.sql
CREATE TABLE posts (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT FALSE
)
​
# down.sql
DROP TABLE posts
​
# 运行up.sql，并且自动生成了schema.rs
diesel migration run
​
# 运行down.sql & up.sql
diesel migration redo
​
