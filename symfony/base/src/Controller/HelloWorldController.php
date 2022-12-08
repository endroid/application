<?php

namespace App\Controller;

use Symfony\Bundle\FrameworkBundle\Controller\AbstractController;
use Symfony\Component\HttpFoundation\Response;
use Symfony\Component\Routing\Annotation\Route;

final class HelloWorldController extends AbstractController
{
    #[Route('/hello-world', name: 'hello_world')]
    public function helloWorldAction(): Response
    {
        return new Response('Hello World');
    }
}
