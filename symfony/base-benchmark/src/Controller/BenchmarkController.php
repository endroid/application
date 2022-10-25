<?php

namespace App\Controller;

use App\Repository\UserRepository;
use Symfony\Bundle\FrameworkBundle\Controller\AbstractController;
use Symfony\Component\HttpFoundation\JsonResponse;
use Symfony\Component\HttpFoundation\Response;
use Symfony\Component\Routing\Annotation\Route;
use Symfony\Component\Serializer\Encoder\JsonEncoder;
use Symfony\Component\Serializer\Normalizer\AbstractNormalizer;
use Symfony\Component\Serializer\Serializer;
use Symfony\Component\Serializer\SerializerInterface;

final class BenchmarkController extends AbstractController
{
    #[Route('/hello-world', name: 'hello_world')]
    public function helloWorldAction(): Response
    {
        return new Response('Hello World');
    }

    #[Route('/repository', name: 'repository')]
    public function repositoryAction(UserRepository $userRepository, SerializerInterface $serializer): Response
    {
        $users = $userRepository->findAll();
        $json = $serializer->serialize(
            $users,
            JsonEncoder::FORMAT,
            [AbstractNormalizer::GROUPS => 'list']
        );

        return new Response($json);
    }
}
