.PHONY: drupal flask laravel nestjs rocket sveltekit symfony

up:
	@make down
	@docker compose build
	@docker compose up -d

login:
	@docker compose exec -ti $(filter-out $@,$(MAKECMDGOALS)) /bin/bash

down:
	@docker stop $$(docker ps -a -q) > /dev/null 2>&1
	@docker network prune --force
	@docker compose down --remove-orphans

drupal:
	@docker compose exec php drupal/bin/$(filter-out $@,$(MAKECMDGOALS))

flask:
	@docker compose exec python flask/bin/$(filter-out $@,$(MAKECMDGOALS))

laravel:
	@docker compose exec php laravel/bin/$(filter-out $@,$(MAKECMDGOALS))

nestjs:
	@docker compose exec node nestjs/bin/$(filter-out $@,$(MAKECMDGOALS))

rocket:
	@docker compose exec rust rocket/bin/$(filter-out $@,$(MAKECMDGOALS))

sveltekit:
	@docker compose exec node sveltekit/bin/$(filter-out $@,$(MAKECMDGOALS))

symfony:
	@docker compose exec php symfony/bin/$(filter-out $@,$(MAKECMDGOALS))

build-all:
	@make drupal build
	@make flask build
	@make laravel build
	@make nestjs build
	@make rocket build
	@make sveltekit build
	@make symfony build

run-all:
	@make drupal run
	@make flask run
	@make laravel run
	@make nestjs run
	@make rocket run
	@make sveltekit run
	@make symfony run

deploy:
	@bin/deploy

benchmark:
	@bin/benchmark

%:
	@:
