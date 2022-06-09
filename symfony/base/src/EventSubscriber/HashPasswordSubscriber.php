<?php

declare(strict_types=1);

namespace App\EventSubscriber;

use App\Entity\User;
use Doctrine\Common\EventSubscriber;
use Doctrine\ORM\Event\LifecycleEventArgs;
use Doctrine\ORM\Events;
use Symfony\Component\PasswordHasher\Hasher\UserPasswordHasherInterface;

final class HashPasswordSubscriber implements EventSubscriber
{
    public function __construct(
        private UserPasswordHasherInterface $passwordHasher
    ) {
    }

    public function getSubscribedEvents(): array
    {
        return [
            Events::prePersist,
            Events::preUpdate,
        ];
    }

    public function prePersist(LifecycleEventArgs $args): void
    {
        $this->updatePassword($args->getEntity());
    }

    public function preUpdate(LifecycleEventArgs $args): void
    {
        $this->updatePassword($args->getEntity());
    }

    private function updatePassword(object $user): void
    {
        if (!$user instanceof User) {
            return;
        }

        if (null === $user->getPlainPassword()) {
            return;
        }

        $password = $this->passwordHasher->hashPassword($user, $user->getPlainPassword());
        $user->setPassword($password);
    }
}
