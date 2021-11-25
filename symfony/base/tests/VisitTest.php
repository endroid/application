<?php

declare(strict_types=1);

namespace App\Tests;

use Symfony\Bundle\FrameworkBundle\Test\WebTestCase;

/**
 * @coversNothing
 */
class VisitTest extends WebTestCase
{
    /**
     * @dataProvider visitData
     * @testdox Check if $path is accessible and contains $search
     */
    public function testVisitPath(string $path, string $search): void
    {
        $client = static::createClient();
        $client->followRedirects(true);
        $client->request('GET', $path);

        $response = $client->getResponse();

        $this->assertResponseStatusCodeSame(200);
        $this->assertStringContainsString($search, strval($response->getContent()));
    }

    /** @return array<array<string, string>> */
    public function visitData(): array
    {
        return [
            ['path' => '/', 'search' => '<div id="app">'],
        ];
    }
}
