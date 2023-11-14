<?php

namespace App\Controller;

use Symfony\Bundle\FrameworkBundle\Controller\AbstractController;
use Symfony\Component\HttpFoundation\Response;
use Symfony\Component\HttpKernel\Attribute\AsController;
use Symfony\Component\Routing\Annotation\Route;

#[AsController]
final class HelloWorldController
{
    #[Route('/hello-world', name: 'hello_world')]
    public function __invoke(): Response
    {
        return new Response('Hello World');
    }
}
