#! /usr/bin/env python3

from dotenv import dotenv_values
import os

database_config = {
    **dotenv_values("database.env")
}

jwt_config = {
    **dotenv_values("jwt.env")
}

database_config['DATABASE_URL'] = "postgres://{username}:{password}@localhost/{database}".format(
    username=database_config['POSTGRES_USER'], password=database_config['POSTGRES_PASSWORD'],
    database=database_config['POSTGRES_DB'])

complete_config = {
    **database_config,
    **jwt_config
}

server_file = open("server/.env", 'w')
infra_dev_file = open("infra/dev/.env", 'w')

for fd in [server_file]:
    for (key, value) in complete_config.items():
        fd.write("{}=\"{}\"\n".format(key, value))

for fd in [infra_dev_file]:
    for (key, value) in database_config.items():
        fd.write("{}=\"{}\"\n".format(key, value))
