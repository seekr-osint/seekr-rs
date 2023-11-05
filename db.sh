#!/bin/sh

rm db/dev.db -f
sqlite3 db/dev.db < ./db/SCHEMA.sql
sqlite3 seekr.sqlx < ./db/SCHEMA.sql 
