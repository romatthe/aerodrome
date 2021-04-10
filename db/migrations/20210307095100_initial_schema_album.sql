create table if not exists album
(
    id varchar(255) not null primary key,
    name varchar(255) default '' not null,
    artist varchar(255) default '' not null
);

create index if not exists album_artist
    on album (artist);

create index if not exists album_name
    on album (name);
