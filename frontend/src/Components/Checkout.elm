module Components.Checkout exposing (..)

import Cart exposing (Cart)
import Html exposing (Html, a, button, div, h2, img, p, span, text)
import Html.Attributes as Attr
import Html.Events exposing (onClick)
import Html.Keyed
import Html.Lazy
import Lib exposing (getItemQuantityPairs, getQuantityElement, getTotal, titleToPath)
import Messages as Msg exposing (Msg)
import Stock exposing (ItemId, Product, Stock)


checkout : { stock : Stock, cart : Cart } -> Html Msg
checkout { stock, cart } =
    div [ Attr.class "absolute min-h-screen top-0 right-0 left-0 min-w-screen bg-gradient-to-b from-kiggypink to-kiggyred" ]
        [ div [ Attr.class "container w-3/4 mx-auto my-8  bg-slate-50" ]
            [ div [ Attr.class "p-8 rounded-md shadow-md" ]
                [ h2 [ Attr.class "text-2xl font-semibold mb-4" ] [ text "cart" ]

                -- Products in cart
                , Html.Keyed.node "div"
                    [ Attr.class "space-y-4" ]
                    (getItemQuantityPairs cart stock |> List.filter (\( _, qty ) -> qty > 0) |> List.map keyedCartItem)

                -- Total and checkout section
                , div [ Attr.class "flex justify-between items-center mt-8" ]
                    [ div []
                        [ p [ Attr.class "text-xl font-semibold" ]
                            [ getTotal cart stock |> String.fromInt |> String.cons '$' |> text ]
                        , p [ Attr.class "text-gray-500 text-sm" ]
                            [ text "shipping: $10" ]
                        ]
                    , button [ Attr.class "bg-kiggygreen text-white px-6 py-2 rounded-md hover:brightness-90", onClick (Msg.GetStripe |> Msg.Nav) ]
                        [ text "checkout" ]
                    ]
                ]
            ]
        ]


keyedCartItem : ( ( ItemId, Product ), Cart.Quantity ) -> ( String, Html Msg )
keyedCartItem (( ( id, _ ), _ ) as args) =
    ( String.fromInt id, Html.Lazy.lazy cartItem args )


cartItem : ( ( ItemId, Product ), Cart.Quantity ) -> Html Msg
cartItem ( ( id, { title, kind, quantity } ), qty ) =
    div [ Attr.class "flex items-center justify-between border-b border-kiggygreen py-4" ]
        -- Left section: Image, title, price
        [ div [ Attr.class "flex items-center space-x-4" ]
            [ a [ Attr.href ("/products/" ++ String.fromInt id) ] [ img [ Attr.class "w-16 h-16 object-cover rounded-md", Attr.src (titleToPath title), Attr.alt ("Product: " ++ title) ] [] ]
            , div []
                [ a [ Attr.href ("/products/" ++ String.fromInt id) ] [ p [ Attr.class "text-gray-800 text-lg font-semibold" ] [ text title ] ]
                , p [ Attr.class "text-gray-500" ] [ kind |> Stock.kindToPrice >> Stock.stringOfPrice |> text ]
                , getQuantityElement quantity
                ]
            ]

        -- Right section: Quantity Controls and remove button
        , div [ Attr.class "flex items-center space-x-4" ]
            [ div [ Attr.class "flex items-center space-x-2" ]
                [ button [ Attr.class "text-gray-600 focus:outline-none", Cart.Dec id |> Msg.Cart |> onClick ]
                    [ span [ Attr.class "text-xl" ] [ text "-" ] ]
                , span [ Attr.class "text-gray-800" ] [ text (String.fromInt qty) ]
                , button [ Attr.class "text-gray-600 focus:outline-none", Cart.Inc id |> Msg.Cart |> onClick ]
                    [ span [ Attr.class "text-xl" ] [ text "+" ] ]
                ]
            , button [ Attr.class "text-red-500 focus:outline-none", Cart.Remove id |> Msg.Cart |> onClick ]
                [ span [ Attr.class "text-md" ] [ text "x" ] ]
            ]
        ]
