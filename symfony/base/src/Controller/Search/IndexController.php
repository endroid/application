<?php

namespace App\Controller\Search;

use Meilisearch\Client;
use Symfony\Component\HttpFoundation\JsonResponse;
use Symfony\Component\HttpFoundation\Response;
use Symfony\Component\HttpKernel\Attribute\AsController;
use Symfony\Component\Routing\Annotation\Route;

#[AsController]
final readonly class IndexController
{
    #[Route('/search/index/{id}/{name}', name: 'search_index')]
    public function __invoke(Client $searchClient, string $id, string $name): Response
    {
        $index = $searchClient->index('default');

        $index->addDocuments([['id' => $id,  'title' => $name]]);

        return new JsonResponse(['success' => true]);
    }
}