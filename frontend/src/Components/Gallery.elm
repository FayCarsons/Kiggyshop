module Components.Gallery exposing (..)

import Components.Dropdown as Dropdown
import Components.Footer exposing (footer)
import Components.Header exposing (header)
import Html exposing (Html, a, div, h2, img, p, text)
import Html.Attributes as Attr
import Lib exposing (getQuantityElement, titleToPath)
import Messages as Msg
import Stock exposing (Product, Stock)


gallery : Stock -> Msg.Menu -> Html Msg.Msg
gallery stock menu =
    div []
        [ div [ Attr.class "relative flex bg-slate-50" ]
            [ Dropdown.dropdown { click = Nothing, class = Dropdown.leftDropdownClass }
            , div [ Attr.class "flex-1 max-w-full" ]
                [ header Msg.FlipMenu menu
                , div [ Attr.class "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4 p-4" ]
                    (List.map productCard stock)
                , footer ()
                ]
            , case menu of
                Msg.Open ->
                    Dropdown.dropdown { click = Just Msg.FlipMenu, class = Dropdown.rightDropdownClass }

                Msg.Closed ->
                    text ""
            ]
        ]


productCard : Product -> Html Msg.Msg
productCard { id, title, kind, quantity } =
    a [ Attr.class "max-w-full overflow-hidden shadow-lg transition duration-300 transform hover:scale-105 aspect-square", Attr.href ("/products/" ++ String.fromInt id) ]
        [ img [ Attr.src (titleToPath title), Attr.class "w-full h-full object-covers transition duration-300 ease-in-out hover:scale-105" ] []
        , div [ Attr.class "absolute inset-0 flex flex-col items-center justify-center bg-white bg-opacity-0 transition duration-300 opacity-0 hover:opacity-100 hover:bg-opacity-75" ]
            [ h2 [ Attr.class "text-kiggypink text-4xl font-semibold mb-2 opacity-100" ] [ text title ]
            , getQuantityElement quantity
            , p [ Attr.class "text-kiggygreen text-2xl opacity-100" ] [ kind |> Stock.kindToPrice |> Stock.stringOfPrice |> text ]
            ]
        ]
