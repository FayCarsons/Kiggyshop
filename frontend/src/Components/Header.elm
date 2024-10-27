module Components.Header exposing (..)

import Components.Icons exposing (Palette(..), burger)
import Html exposing (Html, a, aside, div, hr, text)
import Html.Attributes as Attr
import Messages exposing (Menu(..), Msg(..))


header : Msg -> Menu -> Html Msg
header click showCart =
    Html.header [ Attr.class "md:hidden" ]
        [ div [ Attr.class "bg-gradient-to-r from-kiggyred to-kiggypink p-4 z-0" ]
            [ div [ Attr.class "mx-auto flex justify-center" ]
                [ a [ Attr.class "text-4xl my-1 mx-auto text-white font-bubblegum focus:outline-none", Attr.href "/" ]
                    [ text "KiggyShop" ]
                , case showCart of
                    Closed ->
                        aside []
                            [ burger { click = Just click, class = "md:hidden", size = "24", color = Red } ]

                    Open ->
                        text ""
                ]
            ]
        , hr [ Attr.class "relative m-0 p-0 bg-kiggygreen w-full h-1 border-0 bottom-0 left-0 z-0" ]
            []
        ]
