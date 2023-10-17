-- Add down migration script here
-- drop table pageviews;
-- drop table surveys_submissions;
-- drop table users;
-- drop table surveys;
-- drop table public._sqlx_migrations;

drop table if exists users cascade;
drop table if exists surveys cascade;
drop table if exists questions;
drop table if exists responses;
drop table if exists pageviews;
drop table if exists user_sessions;