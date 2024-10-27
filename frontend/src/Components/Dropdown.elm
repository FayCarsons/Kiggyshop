module Components.Dropdown exposing (..)

{- import Html.Events exposing (onClick) -}

import Components.Icons exposing (Palette(..), burger)
import Html exposing (Html, a, div, text)
import Html.Attributes as Attr
import Messages exposing (Menu(..), Msg(..), Route(..))
import Svg exposing (Svg)


leftDropdownClass : String
leftDropdownClass =
    "w-0 opacity-0 md:opacity-100 md:w-52 transition-all duration-300 ease-in-out bg-kiggygreen flex md:flex-col md:top-0 md:left-0 md:p-4"


rightDropdownClass : String
rightDropdownClass =
    "bg-kiggygreen flex flex-col items-start top-0 right-0 p-4 md:p-0 md:opacity-0 md:w-0 transition-all duration-300 ease-in-out"


intoBurger : Maybe Msg -> Svg Msg
intoBurger onClickMsg =
    case onClickMsg of
        Just _ ->
            burger { click = onClickMsg, class = "absolute top-4 right-4 md:hidden", size = "24", color = Pink }

        Nothing ->
            text ""


dropdown : { showMenu : Menu, click : Maybe Msg, class : String } -> Html Msg
dropdown { showMenu, click, class } =
    case showMenu of
        Open ->
            div [ Attr.class class ]
                [ intoBurger click
                , div [ Attr.class "flex flex-col justify-center border-t border-kiggypink pt-2 mb-4" ]
                    [ a [ Attr.href "/checkout" ] [ text "Checkout" ]
                    , a [ Attr.href "/about" ] [ text "About" ]
                    ]
                ]

        Closed ->
            text ""
