# Database Design

## Environment

* Predict MySQL 8.0

## Tables

links

|name|type|option|
|:---:|:---:|:---:|
|id|bigint|pk, auto increment|
|key|string|not null, unique|
|url|string|not null|
|owner_id|int|not null, fk(users.id)|
|create_at|timestamp||
|update_at|timestamp||

users
|name|type|option|
|:---:|:---:|:---:|
|id|bigint|pk, auto increment|
|email|string|not null, pk|
|screen_name|varchar(64)|not null|
|create_at|timestamp||
|update_at|timestamp||

teams
|name|type|option|
|:---:|:---:|:---:|
|id|bigint|pk, auto increment|
|name|varchar(64)|not null|
|create_at|timestamp||
|update_at|timestamp||


user_teams_relations
|name|type|option|
|:---:|:---:|:---:|
|user_id|int|not null, fk(users.id)|
|team_id|int|not null, fk(users.id)|
|create_at|timestamp||
|update_at|timestamp||
