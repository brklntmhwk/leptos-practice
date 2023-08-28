# Dev Container for Rust

This is a playground for Rust coding where you can work on Rust with ease, consistency, and without worries about installing additional applications on your local to get along with Rust.

## Prerequisites

- You have installed:

  - Docker Desktop
  - VSCode

on your local machine.

- You have added the following VSCode extensions:
  - Dev Containers
  - Docker

## Structure

- .devcontainer
  - devcontainer.json
    - a config file that defines the relationships between dev container and docker container
  - docker-compose.yml
    - a file that creates a Docker container based on the image created in Dockerfile
  - Dockerfile
    - a file that creates a Docker image and customizes which tools to add to the image
- workspace
  - (-- Your Rust Projects here --)

## How to use

1. Run your Docker Desktop
2. Select the menu "Dev Containers: Reopen in Container". (on Windows, press Ctrl + Shift + p to open the dropdown menu)
3. Start your project from scratch or copy your existing one!

e.g. Create a Rust binary project from scratch

```
cargo new [--your project name--]
```

## Other tips

- To exit the container, select the menu "Dev Containers: Reopen in Local".
