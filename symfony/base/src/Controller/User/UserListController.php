<?php

namespace App\Controller\User;

use App\Repository\UserRepository;
use Symfony\Component\HttpFoundation\Response;
use Symfony\Component\HttpKernel\Attribute\AsController;
use Symfony\Component\Routing\Annotation\Route;
use Symfony\Component\Serializer\Encoder\JsonEncoder;
use Symfony\Component\Serializer\Normalizer\AbstractNormalizer;
use Symfony\Component\Serializer\SerializerInterface;

#[AsController]
final readonly class UserListController
{
    public function __construct(
        private UserRepository $userRepository,
        private SerializerInterface $serializer
    ) {}

    #[Route('/user/list', name: 'user_list')]
    public function __invoke(): Response {
        return new Response(
            $this->serializer->serialize(
                $this->userRepository->findAll(),
                JsonEncoder::FORMAT,
                [AbstractNormalizer::GROUPS => 'list']
            )
        );
    }
}
