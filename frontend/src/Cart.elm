module Cart exposing (..)

import Dict exposing (Dict)
import Json.Decode as Decode
import Json.Encode as Encode
import Product exposing (Product)


type alias Id =
    Int


type alias Quantity =
    Int


type alias Cart =
    Dict Id Quantity


type Action
    = Inc Int
    | Dec Int
    | Remove Int
    | Clear


encoder : Cart -> Encode.Value
encoder cart =
    Encode.dict String.fromInt Encode.int cart


getPrice : Dict Int Product -> ( Product.ID, Quantity ) -> Maybe Int
getPrice stock ( id, qty ) =
    Dict.get id stock |> Maybe.map (\product -> Product.kindToPrice product.kind * qty)


getTotal : Cart -> Dict Int Product -> Int
getTotal cart stock =
    Dict.toList cart
        |> List.filterMap (getPrice stock)
        |> List.sum


decoder : Decode.Decoder Cart
decoder =
    Decode.keyValuePairs Decode.int
        |> Decode.andThen
            (\pairs ->
                let
                    addPair : ( String, Int ) -> Decode.Decoder Cart -> Decode.Decoder Cart
                    addPair ( k, v ) decode =
                        case String.toInt k of
                            Just key ->
                                Decode.map (Dict.insert key v) decode

                            Nothing ->
                                Decode.fail "Key must be an integer"
                in
                List.foldr addPair (Decode.succeed Dict.empty) pairs
            )
