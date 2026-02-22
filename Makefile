build:
	@docker compose pull # pulls images defined in compose.yaml
	@docker compose build --pull --no-cache # pulls images defined in Dockerfile

up:
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

benchmark:
	@bin/benchmark
