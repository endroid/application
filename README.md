# Application

## Stack

* Symfony (PHP)
  * RoadRunner (prod) - http://localhost:6001
  * Nginx + RoadRunner (prod) - https://localhost
  * Nginx (dev) - https://localhost:9000
  * Roadrunner Application Server
  * API Platform (incl. GraphQL)
* NestJS (NodeJS / Typescript)
  * Node - http://localhost:3001
  * Nginx - https://localhost:3000
  * TypeORM (incl. fixtures + migrations)
  * GraphQL (Apollo)
* Flask (Python)
  * Nginx - https://localhost:5000
* Rocket (Rust)
  * Nginx - https://localhost:8000
* PostgreSQL
  * Single database server
  * Separate database per service
* Redis
  * Single server
  * Separate namespace per service

## Docker
    
    docker compose up -d

## Build and run

    bin/<service>/build
    bin/<service>/run
