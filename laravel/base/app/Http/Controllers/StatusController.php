<?php

namespace App\Http\Controllers;

class StatusController
{
    public function __invoke()
    {
        return response()->json(['status' => 'ok']);
    }
}
