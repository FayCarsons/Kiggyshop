module App exposing (..)

import Cart exposing (cartDecoder)
import Expect exposing (Expectation)
import Fuzz exposing (Fuzzer, int, list, string)
import Json.Decode exposing (decodeString)
import Stock exposing (stockDecoder)
import Test exposing (..)
import Stock exposing (kindDecoder)
import Stock exposing (kindToPrice)


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
        , test "Valid kind parser" <| 
            \_ -> "\"BigPrint\"" |> decodeString kindDecoder |> Expect.ok 
        , fuzz string "Fuzz kind parser" <| 
            decodeString kindDecoder >> Expect.err
        , test "Kind to price" <| 
            \_ -> Stock.BigPrint |> kindToPrice |> Expect.equal 20
        , test "String of price" <| 
            \_ -> Stock.stringOfPrice 23 |> Expect.equal "$23"
        ]
