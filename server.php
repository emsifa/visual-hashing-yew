<?php

/**
 * PHP BUILT-IN SERVER CONFIG
 * -----------------------------------------
 * We need to customize this because
 * by default PHP built-in server
 * doesn't know how to serve .wasm file
 * -----------------------------------------
 * How to use:
 * -----------------------------------------
 * php -S <host>:<port> -t dist server.php
 * php -S localhost:9090 -t dist server.php
 */

$path = pathinfo($_SERVER["SCRIPT_FILENAME"]);
if ($path["extension"] == "wasm") {
    header("Content-Type: application/wasm");
    readfile($_SERVER["SCRIPT_FILENAME"]);
} else {
    return FALSE;
}
