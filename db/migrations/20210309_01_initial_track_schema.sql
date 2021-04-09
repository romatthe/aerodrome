CREATE TABLE track
(
    id varchar(255) not null primary key,
    title varchar(255) default '' not null,

    artist varchar(255) default '' not null,
    artist_id varchar(255) default '' not null,

    album varchar(255) default '' not null,
    album_id varchar(255) default '' not null,
    album_artist varchar(255) default '' not null,
    album_artist_id varchar(255) default '',
    has_cover_art bool default FALSE not null,
    track_nr integer default 0 not null,
    disc_nr integer default 0 not null,
    disc_subtitle varchar(255),

    year integer default 0 not null,
    size integer default 0 not null,
    suffix varchar(255) default '' not null,
    duration real default 0 not null,
    bitrate integer default 0 not null,
    genre varchar(255) default '' not null,
    full_text varchar(255) default '',
    compilation bool default FALSE not null,
    comment varchar,
    lyrics varchar,
    catalog_nr varchar(255),

    sort_title varchar(255) collate nocase,
    sort_artist_name varchar(255) collate nocase,
    sort_album_name varchar(255) collate nocase,
    sort_album_artist_name varchar(255) collate nocase,

    order_artist_name varchar(255) collate nocase,
    order_album_name varchar(255) collate nocase,
    order_album_artist_name varchar(255) collate nocase,

    mbz_track_id varchar(255),
    mbz_artist_id varchar(255),
    mbz_album_id varchar(255),
    mbz_album_artist_id varchar(255),
    mbz_album_type varchar(255),
    mbz_album_comment varchar(255),

    path varchar(255) default '' not null,
    created_at datetime,
    updated_at datetime
)

