# Application

## Stack

* FrankenPHP + Caddy
* Valkey (cache + sessions)
* PostgreSQL

## Frameworks

* Symfony
* Laravel

## Tooling

* Grafana K6
* Blackfire

## Deployment

```bash
mkdir -p ~/.composer ~/.npm
touch ~/.gitconfig ~/.gitignore_global

rm -rf application
git clone git@github.com:endroid/application.git

rm -rf application/.docker/postgres/data
rm -rf application/.docker/redis/data
```
