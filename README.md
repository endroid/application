# Application

## Stack

### Frameworks

* Symfony (PHP)
  * Caddy + RoadRunner - https://localhost
  * Caddy + PHP-FPM - https://localhost:9000
  * RoadRunner - http://localhost:6001
  * API Platform (incl. GraphQL)
* NestJS (TypeScript)
  * Caddy - https://localhost:3000
  * Node - http://localhost:3001
  * TypeORM (incl. fixtures + migrations)
  * GraphQL (Apollo)
* Fresh (TypeScript)
  * Caddy - https://localhost:3100
  * Deno - http://localhost:3101
* Flask (Python)
  * Caddy - https://localhost:5000
  * Flask - http://localhost:5001
* Rocket (Rust)
  * Caddy - https://localhost:8000
  * Rocket - http://localhost:8001
* Laravel (PHP)
  * Caddy + PHP-FPM - https://localhost:9100
* Drupal (PHP)
  * Caddy - https://localhost:9200

### Services

* PostgreSQL
  * Single database server
  * Separate database per service
* Dragonfly
* Jaeger (OpenTelemetry)
  * UI - http://localhost:16686

## Docker
    
    make up
    make down

## Build and run

    make <image> <framework> build
    make <image> <framework> run
