module Components.Icons exposing (..)

import Svg exposing (..)
import Svg.Attributes as Attr
import Svg.Events exposing (onClick)


type alias SvgWrapperProps msg =
    { click : Maybe msg
    , class : String
    , size : String
    , fill : Maybe Palette
    , stroke : Maybe Palette
    , children : List (Svg msg)
    }


type alias SvgProps msg =
    { click : Maybe msg, class : String, size : String, color : Palette }


type Palette
    = Pink
    | Red
    | Green
    | White
    | Black


type NoOp
    = NoOp


matchPalette : Palette -> ( Int, Int, Int )
matchPalette color =
    case color of
        Pink ->
            ( 255, 142, 173 )

        Red ->
            ( 228, 67, 66 )

        Green ->
            ( 184, 204, 75 )

        White ->
            ( 255, 255, 255 )

        Black ->
            ( 0, 0, 0 )


colorToString : Palette -> String
colorToString color =
    let
        ( r, g, b ) =
            matchPalette color
    in
    "rgb(" ++ String.fromInt r ++ ", " ++ String.fromInt g ++ ", " ++ String.fromInt b ++ ")"


svgWrapper : SvgWrapperProps msg -> Svg msg
svgWrapper { click, class, size, fill, stroke, children } =
    svg
        [ Attr.xmlSpace "http://www.w3.org/2000/svg"
        , case click of
            Just fn ->
                onClick fn

            Nothing ->
                Attr.title ""
        , Attr.width size
        , Attr.height size
        , Attr.fill (Maybe.map colorToString fill |> Maybe.withDefault "none")
        , Attr.stroke (Maybe.map colorToString stroke |> Maybe.withDefault "none")
        , Attr.strokeWidth "2"
        , Attr.strokeLinecap "round"
        , Attr.strokeLinejoin "round"
        , Attr.class class
        ]
        children


burger : SvgProps msg -> Svg msg
burger { click, class, size, color } =
    let
        children =
            [ line [ Attr.x1 "21", Attr.y1 "10", Attr.x2 "3", Attr.y2 "10" ] []
            , line [ Attr.x1 "21", Attr.y1 "6", Attr.x2 "3", Attr.y2 "6" ] []
            , line [ Attr.x1 "21", Attr.y1 "14", Attr.x2 "3", Attr.y2 "14" ] []
            , line [ Attr.x1 "21", Attr.y1 "18", Attr.x2 "3", Attr.y2 "18" ] []
            ]
    in
    svgWrapper { click = click, class = class, size = size, fill = Nothing, stroke = Just color, children = children }


instagram : SvgProps msg -> Svg msg
instagram { click, class, size, color } =
    let
        children =
            [ rect [ Attr.x "2", Attr.y "2", Attr.width "20", Attr.height "20", Attr.rx "5", Attr.ry "5" ] []
            , path [ Attr.d "M16 11.37A4 4 0 1 1 12.63 8 4 4 0 0 1 16 11.37z" ] []
            , line [ Attr.x1 "17.5", Attr.y1 "6.5", Attr.x2 "17.51", Attr.y2 "6.5" ] []
            ]
    in
    svgWrapper { click = click, class = class, size = size, fill = Nothing, stroke = Just color, children = children }


twitter : SvgProps msg -> Svg msg
twitter { click, class, size, color } =
    let
        children =
            [ path [ Attr.d "M23 3a10.9 10.9 0 0 1-3.14 1.53 4.48 4.48 0 0 0-7.86 3v1A10.66 10.66 0 0 1 3 4s-4 9 5 13a11.64 11.64 0 0 1-7 2c9 5 20 0 20-11.5a4.5 4.5 0 0 0-.08-.83A7.72 7.72 0 0 0 23 3z" ] [] ]
    in
    svgWrapper { click = click, class = class, size = size, fill = Nothing, stroke = Just color, children = children }


linkedin : SvgProps msg -> Svg msg
linkedin { click, class, size, color } =
    let
        children =
            [ path [ Attr.d "M16 8a6 6 0 0 1 6 6v7h-4v-7a2 2 0 0 0-2-2 2 2 0 0 0-2 2v7h-4v-7a6 6 0 0 1 6-6z" ] []
            , rect [ Attr.x "2", Attr.width "4", Attr.height "12" ] []
            , circle [ Attr.cx "4", Attr.cy "4", Attr.r "2" ] []
            ]
    in
    svgWrapper { click = click, class = class, size = size, fill = Nothing, stroke = Just color, children = children }


mail : SvgProps msg -> Svg msg
mail { click, class, size, color } =
    let
        children =
            [ path [ Attr.d "M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z" ] []
            , polyline [ Attr.points "22,6 12,13 2,6" ] []
            ]
    in
    svgWrapper { click = click, class = class, size = size, fill = Nothing, stroke = Just color, children = children }

home : SvgProps msg -> Svg msg 
home { click, class, size, color } = 
    let children = [ path [ Attr.d "M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"] [], polyline [ Attr.points "9 22 9 12 15 12 15 22" ] [] ] in
    svgWrapper { click = click, class = class, size = size, fill = Nothing, stroke = Just color, children = children }