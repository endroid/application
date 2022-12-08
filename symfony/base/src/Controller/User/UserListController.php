<?php

namespace App\Controller\User;

use App\Repository\UserRepository;
use Symfony\Bundle\FrameworkBundle\Controller\AbstractController;
use Symfony\Component\HttpFoundation\Response;
use Symfony\Component\Routing\Annotation\Route;
use Symfony\Component\Serializer\Encoder\JsonEncoder;
use Symfony\Component\Serializer\Normalizer\AbstractNormalizer;
use Symfony\Component\Serializer\SerializerInterface;

final class UserListController extends AbstractController
{
    #[Route('/user/list', name: 'user_list')]
    public function __invoke(
        UserRepository $userRepository,
        SerializerInterface $serializer
    ): Response {
        return new Response($serializer->serialize(
            $userRepository->findAll(),
            JsonEncoder::FORMAT,
            [AbstractNormalizer::GROUPS => 'list']
        ));
    }
}
