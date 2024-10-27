module Components.Loading exposing (..)

import Html exposing (Html, div, p, text)
import Html.Attributes as Attr


loadingPage : Html msg
loadingPage =
    div [ Attr.class "flex flex-col items-center justify-center h-screen" ]
        [ div [ Attr.class "animate-spin rounded-full h-16 w-16 border-t-4 border-kiggygreen border-solid" ]
            []
        , p [ Attr.class "mt-4 text-kiggypink text-lg" ]
            [ text "one second >.<" ]
        ]


errorPage : Html msg
errorPage =
    div [] [ text "oh noes :0" ]
