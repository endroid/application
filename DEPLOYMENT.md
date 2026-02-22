# Deployment

## Initial setup

```bash
mkdir -p ~/.composer ~/.npm
touch ~/.gitconfig ~/.gitignore_global
git clone git@github.com:endroid/application.git
```

## Start Docker

```bash
make up
```

## Build applications

```bash
docker compose exec -ti php symfony/bin/build
docker compose exec -ti php laravel/bin/build
```

Now they should be up and running

## Cleaning up

```bash
rm -rf application/.docker/postgres/data
rm -rf application/.docker/redis/data
```
