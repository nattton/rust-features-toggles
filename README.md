# features-toggles

## Installing Diesel CLI

## Linux/MacOS

curl --proto '=https' --tlsv1.2 -LsSf <https://github.com/diesel-rs/diesel/releases/latest/download/diesel_cli-installer.sh> | sh

## Windows

powershell -c "irm <https://github.com/diesel-rs/diesel/releases/latest/download/diesel_cli-installer.ps1> | iex"

## Setup Diesel for your project

diesel setup

## We can apply our new migration:

diesel migration run

## or redo

diesel migration redo
