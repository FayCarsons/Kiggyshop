module Stock exposing (..)

import Cart exposing (Quantity)
import Dict exposing (Dict)
import Http
import Json.Decode as JD
import Product exposing (Kind(..), Product)


type alias Stock =
    Dict Product.ID Product


stockDecoder : JD.Decoder Stock
stockDecoder =
    JD.keyValuePairs Product.decoder
        |> JD.andThen
            (\pairs ->
                let
                    addPair : ( String, Product ) -> JD.Decoder Stock -> JD.Decoder Stock
                    addPair ( k, v ) decoder =
                        case String.toInt k of
                            Just key ->
                                JD.map (Dict.insert key v) decoder

                            Nothing ->
                                JD.fail "Key must be an integer"
                in
                List.foldr addPair (JD.succeed Dict.empty) pairs
            )


findItem : Int -> Stock -> Maybe Product
findItem itemId stock =
    Dict.get itemId stock


