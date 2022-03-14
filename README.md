# Application

## Stack

* Nginx
  * Serves all below services
  * Use HTTPS for each service
* Symfony (PHP) - https://localhost
  * Bootstrap
  * API Platform (incl. GraphQL)
  * Blackfire integration
* NestJS (NodeJS / Typescript) - https://localhost:3000
  * Bootstrap
  * TypeORM (incl. fixtures + migrations)
  * GraphQL (Apollo)
* Flask (Python) - https://localhost:5000
  * Bootstrap
* Rocket (Rust) - https://localhost:8000
  * Bootstrap
* PostgreSQL
  * Single database server
  * Separate database per service
* Redis
  * Single server
  * Separate namespace per service

## Docker
    
    docker-compose up -d

## Build and run

    bin/<service>/build
    bin/<service>/run
