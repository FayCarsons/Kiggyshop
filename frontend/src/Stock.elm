module Stock exposing (..)

import Dict exposing (Dict)
import Http
import Json.Decode as JD


type ProductKind
    = BigPrint
    | SmallPrint
    | Button


type alias Product =
    { title : String
    , description : String
    , kind : ProductKind
    , quantity : Int
    }


type alias ItemId =
    Int


type alias Stock =
    Dict Int Product


type alias StockResult =
    Result Http.Error Stock


kindDecoder : JD.Decoder ProductKind
kindDecoder =
    JD.string
        |> JD.andThen
            (\str ->
                case str of
                    "BigPrint" ->
                        JD.succeed BigPrint

                    "SmallPrint" ->
                        JD.succeed SmallPrint

                    "Button" ->
                        JD.succeed Button

                    other ->
                        JD.fail <| "Unknown product type: " ++ other
            )


productDecoder : JD.Decoder Product
productDecoder =
    JD.map4 Product
        (JD.field "title" JD.string)
        (JD.field "description" JD.string)
        (JD.field "kind" kindDecoder)
        (JD.field "quantity" JD.int)


stockDecoder : JD.Decoder Stock
stockDecoder =
    JD.keyValuePairs productDecoder
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


kindToPrice : ProductKind -> Int
kindToPrice kind =
    case kind of
        BigPrint ->
            20

        SmallPrint ->
            7

        Button ->
            3


stringOfPrice : Int -> String
stringOfPrice price =
    "$" ++ String.fromInt price
