# Application

## Build and run

* NestJS Application
  * TypeORM (incl. fixtures + migrations)
  * GraphQL
* Symfony Application
  * API Platform (incl. GraphQL)
  * Blackfire integration
* Vue Application
* Docker Stack

## Build and run

Run the following commands to have a running application.
    
    docker-compose up -d
    bin/build-and-run

## Guidelines

* Use HTTPS for all served content
* Quality: use CS, Static Analysis, Unit and E2E tests

### NestJS

* Place everything in modules (i.e. users)
* Folder names: controllers, entities etc.
* File names: user-group.entity.ts

## Testing and code quality

* Pragmatic: do not test just to test
* Unit tests: for critical and complex components
* E2E tests using Cypress: also applicable for API tests
* PHP: static analysis using PHPStan and Psalm
* Prettier format for TS and PHP CS Fixer for PHP
