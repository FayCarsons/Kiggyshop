module Components.Dropdown exposing (..)

import Components.Icons exposing (Palette(..), burger)
import Html exposing (Html, a, div, text)
import Html.Attributes as Attr
{- import Html.Events exposing (onClick) -}
import Messages exposing (Msg(..), Route(..))


leftDropdownClass : String
leftDropdownClass =
    "w-0 opacity-0 md:opacity-100 md:w-52 transition-all duration-300 ease-in-out bg-kiggygreen flex md:flex-col md:top-0 md:left-0 md:p-4"


rightDropdownClass : String
rightDropdownClass =
    "bg-kiggygreen flex flex-col items-start top-0 right-0 p-4 md:p-0 md:opacity-0 md:w-0 transition-all duration-300 ease-in-out"


dropdown : { click : Maybe Msg, class : String } -> Html Msg
dropdown { click, class } =
    div [ Attr.class class ]
        [ click
            |> (burger
                    { click = click
                    , class = "absolute top-4 right-4 md:hidden"
                    , size = "24"
                    , color = Pink
                    }
                    |> Just
                    |> always
                    |> Maybe.andThen
               )
            |> Maybe.withDefault (text "")
        , div [ Attr.class "flex flex-col justify-center border-t border-kiggypink pt-2 mb-4" ]
            [ a [ Attr.href "/checkout" ] [ text "Checkout" ]
            , a [ Attr.href "/about" ] [ text "About" ]
            ]
        ]
