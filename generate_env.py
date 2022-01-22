#! /usr/bin/env python3

from dotenv import dotenv_values
import os

complete_config = {
    **dotenv_values("database.env")
}

complete_config['DATABASE_URL'] = "DATABASE_URL=postgres://{username}:{password}@localhost/{database}".format(username = complete_config['POSTGRES_USER'], password = complete_config['POSTGRES_PASSWORD'], database = complete_config['POSTGRES_DB'])

server_file = open("server/.env", 'w')
infra_dev_file = open("infra/dev/.env", 'w')

for fd in [server_file, infra_dev_file]:
    for (key, value) in complete_config.items():
        fd.write("{}=\"{}\"\n".format(key, value))