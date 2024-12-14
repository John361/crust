#!/bin/bash

# Variables
install_directory="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"
current_directory="$(pwd)"

services_directory="/etc/systemd/system"
working_directory="/etc/crust"
program_name="crust"


# Build
cargo build --release
sudo cp "${current_directory}/target/release/${program_name}" /usr/local/sbin


# Prepare config
sudo mkdir -p "${working_directory}"
sudo touch "${working_directory}/${program_name}.yaml"
sudo cp "${current_directory}/config/log4rs.yaml" "${working_directory}"

sudo chmod 700 -R "${working_directory}"
sudo chown root:root -R "${working_directory}"


# Prepare service
sudo cp "${install_directory}/config/${program_name}.service" "${services_directory}/${program_name}.service"
sudo chmod 644 "${services_directory}/${program_name}.service"
sudo chown root:root "${services_directory}/${program_name}.service"

sudo systemctl daemon-reload
sudo systemctl enable "${program_name}"
