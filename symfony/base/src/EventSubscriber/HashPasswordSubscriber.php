<?php

declare(strict_types=1);

namespace App\EventSubscriber;

use App\Entity\User;
use Doctrine\ORM\Event\PrePersistEventArgs;
use Doctrine\ORM\Events;
use Doctrine\ORM\Event\PreUpdateEventArgs;
use Symfony\Component\EventDispatcher\Attribute\AsEventListener;
use Symfony\Component\PasswordHasher\Hasher\UserPasswordHasherInterface;

#[AsEventListener(event: Events::prePersist)]
#[AsEventListener(event: Events::preUpdate)]
final readonly class HashPasswordSubscriber
{
    public function __construct(
        private UserPasswordHasherInterface $passwordHasher
    ) {
    }

    public function onPrePersist(PrePersistEventArgs $args): void
    {
        $this->updatePassword($args->getObject());
    }

    public function onPreUpdate(PreUpdateEventArgs $args): void
    {
        $this->updatePassword($args->getObject());
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
