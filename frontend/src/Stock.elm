module Stock exposing (..)

import Http
import Json.Decode as JD


type ProductKind
    = BigPrint
    | SmallPrint
    | Button


type alias Product =
    { id : Int
    , title : String
    , description : String
    , kind : ProductKind
    , quantity : Int
    }


type alias Stock =
    List Product


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
    JD.map5 Product
        (JD.field "id" JD.int)
        (JD.field "title" JD.string)
        (JD.field "description" JD.string)
        (JD.field "kind" kindDecoder)
        (JD.field "quantity" JD.int)


stockDecoder : JD.Decoder Stock
stockDecoder =
    JD.list productDecoder


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
