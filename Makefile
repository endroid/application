.PHONY: django drupal fiber laravel nestjs rocket sveltekit symfony

up:
	@make down
	@docker compose pull
	@docker compose build
	@docker compose up -d --force-recreate

login:
	@docker compose exec -ti $(filter-out $@,$(MAKECMDGOALS)) /bin/bash

login-root:
	@docker compose exec --user=0 -ti $(filter-out $@,$(MAKECMDGOALS)) /bin/bash

down:
	@docker stop $$(docker ps -a -q) > /dev/null 2>&1
	@docker network prune --force
	@docker compose down --remove-orphans

prune:
	@docker system prune -a --volumes

certbot:
	@docker compose exec nginx /etc/nginx/ssl/certbot

django:
	@docker compose exec python django/bin/$(filter-out $@,$(MAKECMDGOALS))

drupal:
	@docker compose exec php drupal/bin/$(filter-out $@,$(MAKECMDGOALS))

fiber:
	@docker compose exec golang fiber/bin/$(filter-out $@,$(MAKECMDGOALS))

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
	@make django build
	@make drupal build
	@make fiber build
	@make laravel build
	@make nestjs build
	@make rocket build
	@make sveltekit build
	@make symfony build

run-all:
	@make django run
	@make drupal run
	@make fiber run
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
