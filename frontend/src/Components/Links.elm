module Components.Links exposing (..)

import Components.Icons exposing (Palette(..), instagram, linkedin, mail, twitter)
import Html exposing (Html, a, div)
import Html.Attributes as Attr


type LinkSize
    = Small
    | Large


links : { size : LinkSize, class : String } -> Html msg
links { size, class } =
    let
        ( svgSize, svgClass ) =
            case size of
                Large ->
                    ( "24", "w-8 h-8" )

                Small ->
                    ( "12", "w-4 h-4" )
    in
    let
        props =
            { click = Nothing, class = svgClass, size = svgSize, color = Pink }
    in
    div [ Attr.class class ]
        [ a [ Attr.href "https://www.instagram.com/k1ggy", Attr.target "_blank", Attr.rel "noopener noreeferrer" ]
            [ instagram props ]
        , a [ Attr.href "https://www.instagram.com/k1ggy", Attr.target "_blank", Attr.rel "noopener noreeferrer" ]
            -- And here and in the rest ! Don't forget :^)
            [ twitter props ]
        , a [ Attr.href "https://www.instagram.com/k1ggy", Attr.target "_blank", Attr.rel "noopener noreeferrer" ]
            [ linkedin props ]
        , a [ Attr.href "https://www.instagram.com/k1ggy", Attr.target "_blank", Attr.rel "noopener noreeferrer" ]
            [ mail props ]
        ]
