<?php

namespace App\DataFixtures;

use App\Entity\User;
use Doctrine\Bundle\FixturesBundle\Fixture;
use Doctrine\Persistence\ObjectManager;
use Symfony\Component\Uid\UuidV4;
use Symfony\Component\Yaml\Yaml;

class UserFixture extends Fixture
{
    public function load(ObjectManager $manager): void
    {
        $data = Yaml::parseFile(__DIR__.'/../../fixtures/user.yaml');

        foreach ($data as $item) {
            $user = new User(
                id: UuidV4::fromString($item['id']),
                email: $item['email'],
                roles: $item['roles'],
            );
            $user->setPlainPassword($item['plainPassword']);
            $manager->persist($user);
        }

        $manager->flush();
    }
}
