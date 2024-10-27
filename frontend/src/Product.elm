module Product exposing (ID, Kind(..), Product, decoder, formattedPrice, kindToPrice)

import Json.Decode as Decode


type Kind
    = BigPrint
    | SmallPrint
    | Button


kindToPrice : Kind -> Int
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


formattedPrice : Kind -> String
formattedPrice =
    kindToPrice >> stringOfPrice


type alias Product =
    { title : String
    , description : String
    , kind : Kind
    , quantity : Int
    }


type alias ID =
    Int


kindDecoder : Decode.Decoder Kind
kindDecoder =
    Decode.string
        |> Decode.andThen
            (\str ->
                case str of
                    "BigPrint" ->
                        Decode.succeed BigPrint

                    "SmallPrint" ->
                        Decode.succeed SmallPrint

                    "Button" ->
                        Decode.succeed Button

                    other ->
                        Decode.fail <| "Unknown product type: " ++ other
            )


decoder : Decode.Decoder Product
decoder =
    Decode.map4 Product
        (Decode.field "title" Decode.string)
        (Decode.field "description" Decode.string)
        (Decode.field "kind" kindDecoder)
        (Decode.field "quantity" Decode.int)
