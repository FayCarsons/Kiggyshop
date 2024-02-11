module Cart exposing (..)

import Dict exposing (Dict)
import Json.Decode as JD
import Json.Encode as JE


type alias Id =
    Int


type alias Quantity =
    Int


type alias Cart =
    Dict Id Quantity


type CartAction
    = Inc Int
    | Dec Int
    | Remove Int
    | Clear


type alias CartResult =
    Result JD.Error Cart


cartEncoder : Cart -> JE.Value
cartEncoder cart =
    JE.dict String.fromInt JE.int cart


cartDecoder : JD.Decoder Cart
cartDecoder =
    JD.keyValuePairs JD.int
        |> JD.andThen
            (\pairs ->
                let
                    addPair : ( String, Int ) -> JD.Decoder Cart -> JD.Decoder Cart
                    addPair ( k, v ) decoder =
                        case String.toInt k of
                            Just key ->
                                JD.map (Dict.insert key v) decoder

                            Nothing ->
                                JD.fail "Key must be an integer"
                in
                List.foldr addPair (JD.succeed Dict.empty) pairs
            )
