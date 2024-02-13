module Components.LazyImage exposing (..)
import Html exposing (Html, div, img)

lazyImage : List (Html.Attribute msg) -> List (Html msg) -> Html msg
lazyImage attrs children = 
    div []
        [ img [] []]
