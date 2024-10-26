module Components.Product exposing (..)

import Cart exposing (CartAction(..))
import Components.Dropdown as Dropdown
import Components.Header exposing (header)
import Html exposing (Html, button, div, h1, img, p, text)
import Html.Attributes as Attr
import Html.Events exposing (onClick)
import Html.Lazy
import Lib exposing (getQuantityElement, titleToPath)
import Messages as Msg exposing (Msg)
import Stock exposing (ItemId, Product)


product : ItemId -> Product -> Msg.Menu -> Html Msg
product id item menu =
    div [ Attr.class "relative flex min-h-screen bg-slate-50" ]
        [ Dropdown.dropdown { showMenu = Msg.Open, click = Nothing, class = Dropdown.leftDropdownClass }
        , div [ Attr.class "flex flex-col min-h-screen" ]
            [ header Msg.FlipMenu menu
            , productPage id item
            , Html.Lazy.lazy Dropdown.dropdown { showMenu = menu, click = Just Msg.FlipMenu, class = Dropdown.rightDropdownClass }
            ]
        ]


productPage : ItemId -> Product -> Html Msg
productPage id { title, description, kind, quantity } =
    div [ Attr.class "flex flex-col items-center md:flex-row md:justify-center" ]
        [ div [ Attr.class "md:w-1/2 p-4 flex flex-col items-center justify-center" ]
            [ img [ title |> titleToPath |> Attr.src, Attr.alt title, Attr.class "w-full h-auto object-cover lg" ] [] ]
        , div [ Attr.class "md: w-1/2 p-4 text-center md:text-left" ]
            [ h1 [ Attr.class "text-3xl font-semibold mb-2" ] [ text title ]
            , p [ Attr.class "text-gray-700 mb-4" ] [ text description ]
            , div [ Attr.class "flex flex-col items-center justify-center md:items-start md:justify-start mb-4" ]
                [ p [ Attr.class "text-lg font-semibold text-gray-900 mr-2 md:left-0" ] [ kind |> Stock.kindToPrice >> Stock.stringOfPrice >> text ]
                , getQuantityElement quantity
                ]
            , if quantity > 0 then
                button
                    [ Attr.class "bg--kiggypink brightness-100 text-white py-2 px-4 md:px-6 rounded transiition duration-300 ease-in-out hover:brightness-90 focus:ring focus:ring-kiggypink"
                    , id |> Cart.Inc |> Msg.Cart |> onClick
                    ]
                    [ text "add to cart" ]

              else
                text ""
            ]
        ]
