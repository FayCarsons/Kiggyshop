module App exposing (..)

import Cart
import Expect
import Fuzz exposing (Fuzzer, int, list, string)
import Json.Decode exposing (decodeString)
import Product
import Stock exposing (stockDecoder)
import Test exposing (..)


suite : Test
suite =
    describe "App tests"
        [ test "Valid cart parser" <|
            \_ -> "{ \"1\": 0, \"2\": 1 }" |> decodeString Cart.decoder |> Expect.ok
        , fuzz string "Fuzz cart parser" <|
            \s -> decodeString Cart.decoder s |> Expect.err
        , test "Valid stock parser" <|
            \_ ->
                """{"1": {"title":"cat","kind":"BigPrint","description":"8x17 print from a local print shop with high quality 100lb silk cover paper","quantity":20}}"""
                    |> decodeString stockDecoder
                    |> Expect.ok
        , fuzz string "Fuzz stock parser" <|
            \s -> decodeString stockDecoder s |> Expect.err
        , test "Valid kind parser" <|
            \_ -> "\"BigPrint\"" |> decodeString Product.kindDecoder |> Expect.ok
        , fuzz string "Fuzz kind parser" <|
            decodeString Product.kindDecoder
                >> Expect.err
        , test "Kind to price" <|
            \_ -> Product.BigPrint |> Product.kindToPrice |> Expect.equal 20
        ]
