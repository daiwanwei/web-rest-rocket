# web-rest-rocket

## Description

**example of REST API for rocket.rs**

## How To Run This Project

### edit database uri
```text
/*edit Rocket.toml*/
diesel = { url = "postgres://{DB_username}:{DB_password}@localhost/rest_example"}
```

### Build

```shell
cargo build
```

### Run

```shell
cargo run
```

## How To migrate and edit database

### edit .env

```text
DATABASE_URL=postgres://{DB_username}:{{DB_password}}@localhost/rest_example
```

### migrate database

```shell
diesel setup 
diesel migration run

#redo
diesel migration redo
```

### generate new record

```shell
diesel migration generate {action_name}
#add sql in up.sql and down.sql
#then->
diesel migration run
```

