# Application

## Stack

* Symfony (PHP)
  * Nginx + RoadRunner - https://localhost
  * Nginx - https://localhost:9000
  * RoadRunner - http://localhost:6001
  * API Platform (incl. GraphQL)
* NestJS (NodeJS + TypeScript)
  * Nginx - https://localhost:3000
  * Node - http://localhost:3001
  * TypeORM (incl. fixtures + migrations)
  * GraphQL (Apollo)
* Flask (Python)
  * Nginx - https://localhost:5000
  * Flask - http://localhost:5001
* Rocket (Rust)
  * Nginx - https://localhost:8000
  * Rocket - http://localhost:8001
* Drupal (PHP)
  * Nginx - https://localhost:9100
* Laravel (PHP)
  * Nginx - https://localhost:9200
* PostgreSQL
  * Single database server
  * Separate database per service
* Redis
  * Single server
  * Separate namespace per service

## Docker
    
    make up
    make down

## Build and run

    make <service> build
    make <service> run
