<?php

namespace App\Security;

use Symfony\Component\Security\Core\Exception\AuthenticationException;
use Symfony\Component\Security\Http\AccessToken\AccessTokenHandlerInterface;
use Symfony\Component\Security\Http\Authenticator\Passport\Badge\UserBadge;

final readonly class ApiTokenHandler implements AccessTokenHandlerInterface
{
    public function __construct(
        private UserProvider $userProvider
    ) {
    }

    public function getUserBadgeFrom(#[\SensitiveParameter] string $accessToken): UserBadge
    {
        try {
            $user = $this->userProvider->loadByToken($accessToken);

            return new UserBadge($user->getUserIdentifier(), fn () => $user);
        } catch (\Exception $e) {
            throw new AuthenticationException('Could not authenticate user: ' . $e->getMessage());
        }
    }
}
