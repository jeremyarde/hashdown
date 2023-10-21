-- Add down migration script here
drop table if exists mdp.users cascade;
drop table if exists mdp.surveys cascade;
drop table if exists mdp.questions;
drop table if exists mdp.responses;
drop table if exists mdp.pageviews;
drop table if exists mdp.sessions;