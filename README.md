# Application

## Stack

* Symfony (PHP)
  * RoadRunner - https://localhost
  * Nginx - https://localhost:9000
  * Bootstrap
  * Roadrunner Application Server
  * API Platform (incl. GraphQL)
  * Blackfire integration
* NestJS (NodeJS / Typescript)
  * Nginx - https://localhost:3000
  * Bootstrap
  * TypeORM (incl. fixtures + migrations)
  * GraphQL (Apollo)
* Flask (Python)
  * Nginx - https://localhost:5000
  * Bootstrap
* Rocket (Rust)
  * Nginx - https://localhost:8000
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
