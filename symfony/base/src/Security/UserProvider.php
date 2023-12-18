<?php

namespace App\Security;

use App\Model\User;
use FusionAuth\ClientResponse;
use FusionAuth\FusionAuthClient;
use Symfony\Component\Security\Core\User\UserInterface;
use Symfony\Component\Security\Core\User\UserProviderInterface;

final readonly class UserProvider implements UserProviderInterface
{
    public function __construct(
        private FusionAuthClient $fusionAuthClient,
        private RoleMapper $roleMapper
    ) {
    }

    public function refreshUser(UserInterface $user): UserInterface
    {
        die('refresh');
    }

    public function supportsClass(string $class): bool
    {
        die('supports');
    }

    public function loadUserByIdentifier(string $identifier): UserInterface
    {
        die('load');
    }

    public function loadByToken(string $token): UserInterface
    {
        $fusionAuthResponse = $this->fusionAuthClient->retrieveUserUsingJWT($token);

        return $this->createUserFromFusionAuthResponse($fusionAuthResponse);
    }

    private function createUserFromFusionAuthResponse(ClientResponse $fusionAuthResponse): UserInterface
    {
        $user = $fusionAuthResponse->successResponse->user;

        return new User(
            $user->id,
            $user->email,
            $this->roleMapper->mapRoles($user->registrations)
        );
    }
}
