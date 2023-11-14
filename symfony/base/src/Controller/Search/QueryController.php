<?php

namespace App\Controller\Search;

use Meilisearch\Client;
use Symfony\Component\HttpFoundation\JsonResponse;
use Symfony\Component\HttpFoundation\Response;
use Symfony\Component\HttpKernel\Attribute\AsController;
use Symfony\Component\Routing\Annotation\Route;

#[AsController]
final readonly class QueryController
{
    #[Route('/search/query/{query}', name: 'search_query')]
    public function __invoke(Client $searchClient, string $query): Response
    {
        $index = $searchClient->index('default');

        $hits = $index->search($query)->getHits();

        return new JsonResponse($hits);
    }
}