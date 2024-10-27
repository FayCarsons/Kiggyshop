module Components.Gallery exposing (..)

import Components.Dropdown as Dropdown
import Components.Footer exposing (footer)
import Components.Header exposing (header)
import Dict
import Html exposing (Html, a, div, h2, img, p, text)
import Html.Attributes as Attr
import Html.Keyed
import Html.Lazy
import Lib exposing (getQuantityElement, titleToPath)
import Messages exposing (Menu(..), Msg(..))
import Product exposing (Product)
import Stock exposing (Stock)


gallery : Stock -> Menu -> Html Msg
gallery stock menu =
    div []
        [ div [ Attr.class "relative flex bg-slate-50" ]
            [ Dropdown.dropdown { showMenu = Open, click = Nothing, class = Dropdown.leftDropdownClass }
            , div [ Attr.class "flex-1 max-w-full" ]
                [ header FlipMenu menu
                , Html.Keyed.node "div"
                    [ Attr.class "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4 p-4" ]
                    (Dict.toList stock |> List.map keyedProductCard)
                , footer ()
                ]
            , Html.Lazy.lazy Dropdown.dropdown { showMenu = menu, click = Just FlipMenu, class = Dropdown.rightDropdownClass }
            ]
        ]


keyedProductCard : ( Product.ID, Product ) -> ( String, Html Msg )
keyedProductCard (( id, _ ) as pair) =
    ( String.fromInt id, Html.Lazy.lazy productCard pair )


productCard : ( Int, Product ) -> Html Msg
productCard ( id, { title, kind, quantity } ) =
    a [ Attr.class "max-w-full overflow-hidden shadow-lg transition duration-300 transform hover:scale-105 aspect-square", Attr.href ("/products/" ++ String.fromInt id) ]
        [ img [ Attr.src (titleToPath title), Attr.class "w-full h-full object-cover transition duration-300 ease-in-out hover:scale-105" ] []
        , div [ Attr.class "absolute inset-0 flex flex-col items-center justify-center bg-white bg-opacity-0 transition duration-300 opacity-0 hover:opacity-100 hover:bg-opacity-75" ]
            [ h2 [ Attr.class "text-kiggypink text-4xl font-semibold mb-2 opacity-100" ] [ text title ]
            , getQuantityElement quantity
            , p [ Attr.class "text-kiggygreen text-2xl opacity-100" ]
                [ text
                    (Product.formattedPrice kind)
                ]
            ]
        ]
