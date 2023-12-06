-- Add migration script here
CREATE TABLE todo
(
    id         uuid    DEFAULT gen_random_uuid() PRIMARY KEY NOT NULL ,
    title      TEXT    NOT NULL,
    content    text    NOT NULL,
    completed  boolean DEFAULT false NOT NULL,
    created_at timestamp with time zone  NOT NULL DEFAULT current_timestamp
);