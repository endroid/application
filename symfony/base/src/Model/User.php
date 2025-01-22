<?php

namespace App\Model;

use Symfony\Component\Security\Core\User\UserInterface;

final readonly class User implements UserInterface
{
    public function __construct(
        private string $userIdentifier,
        private string $email,
        private array $roles
    ) {
    }

    public function getUserIdentifier(): string
    {
        if ('' === $this->userIdentifier) {
            throw new \LogicException('The user identifier should always be a non-empty string');
        }

        return $this->userIdentifier;
    }

    public function getEmail(): string
    {
        return $this->email;
    }

    public function getRoles(): array
    {
        return $this->roles;
    }

    public function eraseCredentials(): void
    {
        // not implemented
    }
}
