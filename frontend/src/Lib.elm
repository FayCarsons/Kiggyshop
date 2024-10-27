module Lib exposing (..)

import Cart exposing (Cart)
import Dict
import Html exposing (Html, p, text)
import Html.Attributes as Attr
import Http
import Messages exposing (Msg(..), Nav(..))
import Product exposing (Product)
import Stock exposing (Stock)


inc : number -> number
inc n =
    n + 1


dec : number -> number
dec n =
    n - 1


throw : a -> ()
throw err =
    let
        _ =
            Debug.log "ERROR: " err
    in
    ()


titleToPath : String -> String
titleToPath s =
    "/images/" ++ (String.trim >> String.replace " " "") s ++ ".png"


boundQuantityInStock : (Int -> Int) -> Cart.Quantity -> Product -> Int
boundQuantityInStock op qty item =
    min item.quantity (op qty) |> max 0


mapCart : (Cart.Quantity -> Cart.Quantity) -> Product -> Maybe Cart.Quantity -> Maybe Cart.Quantity
mapCart op product qty =
    case qty of
        Just amt ->
            Just (boundQuantityInStock op amt product)

        Nothing ->
            Just 1


getQuantityElement : Int -> Html msg
getQuantityElement qty =
    if qty == 0 then
        p [ Attr.class "text-kiggyred mb-2" ] [ text "out of stock :/" ]

    else if qty <= 10 then
        p [ Attr.class "text-kiggyred mb-2" ] [ text ("only " ++ String.fromInt qty ++ " left in stock!") ]

    else
        text ""


getItemQuantityPairs : Cart -> Stock -> List ( ( Product.ID, Product ), Cart.Quantity )
getItemQuantityPairs cart stock =
    Dict.toList cart |> List.filterMap (\( id, qty ) -> Dict.get id stock |> Maybe.andThen (\item -> Just ( ( id, item ), qty )))


postCheckout : Cart -> Cmd Msg
postCheckout cart =
    Http.post
        { url = "/api/checkout"
        , body = Http.jsonBody (Cart.encoder cart)
        , expect = Http.expectString (\s -> Nav (GotStripe s))
        }
