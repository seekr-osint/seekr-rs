#!/bin/sh
sea-orm-cli generate entity -o entity/src && mv entity/src/mod.rs  entity/src/lib.rs
