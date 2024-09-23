# Guidelines

* Pragmatism over perfection
  * As long as you deliver quality (i.e. write tests)
  * You can always improve what you have
* Never expose internals
  * Law of demeter: assume as little as possible about externals
  * Open/closed principle: open for extension, closed for modification
* Composition over inheritance
  * Don't use inheritance just to reuse code and avoid duplication
  * Composition improves encapsulation and maintainability
* Naming conventions
  * Use meaningful and descriptive names
  * Avoid names that are too generic (like "Helper" or "Service")
  * Methods should always contain a verb (they do something)
  * Abstract classes are prefixed by Abstract (PHP-FIG)
  * Interfaces are suffixed by Interface (PHP-FIG)
* Write explicit code and keep away from magic
  * The more you see what happens, the more you know what you are doing
  * Magic is difficult to debug and vulnerable to changes
* Apply immutability where possible
  * An object should never exist in an invalid state
  * Don't rely on background generated IDs upon DB persistence
  * Avoid using setters and mutable properties
  * Inject all required properties via the constructor
    * An instantiated object has to be a valid / viable object
    * Do not allow a temporary invalid state by design
    * For instance: use DTOs in forms and keep your model clean
* Rely on abstractions, not on concrete implementations
* Use voters to enforce proper authorization
  * Keep high level checks simple by voting on attributes and subjects
  * Keep rules central by applying role checks and logic inside voters
