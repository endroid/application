.PHONY: bevy django fiber fresh laravel nestjs rocket sveltekit symfony

ENDROID_CMD := $(wordlist 2,$(words $(MAKECMDGOALS)),$(MAKECMDGOALS))

build:
	@docker compose pull
	@docker compose build --pull --no-cache

up:
	@make down
	@docker compose build --pull
	@docker compose up -d --force-recreate --remove-orphans

down:
	@docker stop $$(docker ps -a -q) > /dev/null 2>&1
	@docker network prune --force
	@docker compose down --remove-orphans

prune:
	@docker system prune -a --volumes

login:
	@docker compose exec -ti $(filter-out $@,$(MAKECMDGOALS)) /bin/bash

login-root:
	@docker compose exec --user=0 -ti $(filter-out $@,$(MAKECMDGOALS)) /bin/bash

install-ca:
	.docker/caddy/ssl/install-ca

deploy:
	@bin/deploy

endroid:
	@for dir in symfony/application/vendor/endroid/*; do \
		(cd $$dir && $(ENDROID_CMD)); \
	done

benchmark:
	@bin/benchmark

%:
	@:
