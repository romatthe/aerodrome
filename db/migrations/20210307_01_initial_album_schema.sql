create table if not exists album
(
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name varchar(255) default '' not null,
    artist varchar(255) default '' not null
);

create index if not exists album_artist
    on album (artist);

create index if not exists album_name
    on album (name);
