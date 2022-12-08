.PHONY: flask nestjs sveltekit rocket symfony

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

flask:
	@docker compose exec python flask/bin/$(filter-out $@,$(MAKECMDGOALS))

nestjs:
	@docker compose exec node nestjs/bin/$(filter-out $@,$(MAKECMDGOALS))

sveltekit:
	@docker compose exec node sveltekit/bin/$(filter-out $@,$(MAKECMDGOALS))

rocket:
	@docker compose exec rust rocket/bin/$(filter-out $@,$(MAKECMDGOALS))

symfony:
	@docker compose exec php symfony/bin/$(filter-out $@,$(MAKECMDGOALS))

build-all:
	@make flask build
	@make nestjs build
	@make sveltekit build
	@make rocket build
	@make symfony build

run-all:
	@make flask run
	@make nestjs run
	@make sveltekit run
	@make rocket run
	@make symfony run

deploy:
	@bin/deploy

benchmark:
	@bin/benchmark

%:
	@:
