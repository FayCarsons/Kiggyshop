module Components.LazyImage exposing (..)

import Html exposing (Html, div, img)



{- Write this as a web-component so you don't have to deal w/ ports -}


lazyImage : List (Html.Attribute msg) -> List (Html msg) -> Html msg
lazyImage attrs children =
    div []
        [ img [] [] ]
