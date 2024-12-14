#!/bin/bash

# Variables
services_directory="/etc/systemd/system"
working_directory="/etc/crust"
program_name="crust"


# Uninstall
sudo systemctl stop "${program_name}"
sudo systemctl disable "${program_name}"

sudo rm -f "/usr/local/sbin/${program_name}"
sudo rm -f "${services_directory}/${program_name}.service"
sudo rm -rf "${working_directory}"

sudo systemctl daemon-reload
sudo systemctl reset-failed
