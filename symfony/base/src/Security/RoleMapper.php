<?php

namespace App\Security;

final class RoleMapper
{
    private array $mapping = [
        'member' => Role::MEMBER,
        'admin' => Role::ADMIN,
    ];

    public function __construct(
        private readonly string $applicationId
    ) {
    }

    public function mapRoles(array $registrations): array
    {
        $roles = [];
        foreach ($registrations as $registration) {
            if ($registration->applicationId === $this->applicationId) {
                foreach ($registration->roles as $role) {
                    if (isset($this->mapping[$role])) {
                        $roles[] = $this->mapping[$role];
                    }
                }
            }
        }

        return $roles;
    }
}
