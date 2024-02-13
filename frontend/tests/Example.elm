module Example exposing (..)

import Cart exposing (cartDecoder)
import Expect exposing (Expectation)
import Fuzz exposing (Fuzzer, int, list, string)
import Json.Decode exposing (decodeString)
import Stock exposing (stockDecoder)
import Test exposing (..)


suite : Test
suite =
    describe "App tests"
        [ test "Valid cart parser" <|
            \_ -> "{ \"1\": 0, \"2\": 1 }" |> decodeString cartDecoder |> Expect.ok
        , fuzz string "Fuzz cart parser" <|
            \s -> decodeString cartDecoder s |> Expect.err
        , test "Valid stock parser" <| 
            \_ -> """{"1": {"title":"cat","kind":"BigPrint","description":"8x17 print from a local print shop with high quality 100lb silk cover paper","quantity":20}}""" 
                |> decodeString stockDecoder 
                |> Expect.ok
        , fuzz string "Fuzz stock parser" <|
            \s -> decodeString stockDecoder s |> Expect.err
        ]
