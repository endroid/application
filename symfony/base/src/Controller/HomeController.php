<?php

namespace App\Controller;

use Symfony\Component\HttpFoundation\Response;
use Symfony\Component\HttpKernel\Attribute\AsController;
use Symfony\Component\Routing\Annotation\Route;
use Twig\Environment;

#[AsController]
final class HomeController
{
    #[Route('/', name: 'home')]
    public function __invoke(Environment $renderer): Response
    {
        return new Response($renderer->render('home.html.twig'));
    }
}
