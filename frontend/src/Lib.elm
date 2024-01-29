module Lib exposing (..)

import Cart exposing (Cart, Quantity, cartEncoder)
import Dict
import Html exposing (Html, p, text)
import Html.Attributes as Attr
import Http
import Messages as Msg
import Stock exposing (Product, Stock)


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


get : Int -> List a -> Maybe a
get n l =
    List.head (List.drop n l)


titleToPath : String -> String
titleToPath s =
    "/resources/images/" ++ (String.trim >> String.replace " " "") s ++ ".png"


findItem : Int -> Stock -> Maybe Product
findItem itemId stock =
    case List.filter (\{ id } -> id == itemId) stock of
        [ item ] ->
            Just item

        _ ->
            Nothing


boundQuantityInStock : (Int -> Int) -> Quantity -> Product -> Int
boundQuantityInStock op qty item =
    let
        res =
            op qty
    in
    min item.quantity res |> max 0


mapCart : (Quantity -> Quantity) -> Product -> Maybe Quantity -> Maybe Quantity
mapCart op product qty =
    case qty of
        Just amt ->
            Just <| boundQuantityInStock op amt product

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


getItemQuantityPairs : Cart -> Stock -> List ( Product, Cart.Quantity )
getItemQuantityPairs cart stock =
    List.filterMap
        (\({ id } as item) ->
            Dict.get id cart |> Maybe.andThen (\qty -> Just ( item, qty ))
        )
        stock


getTotal : Cart -> Stock -> Int
getTotal cart stock =
    List.sum (List.filterMap (\{ id, kind } -> Dict.get id cart |> Maybe.map (\qty -> qty * Stock.kindToPrice kind)) stock)


postCheckout : Cart -> Cmd Msg.Msg
postCheckout cart =
    Http.post
        { url = "buttbuddy"
        , body = Http.jsonBody (cartEncoder cart)
        , expect = Http.expectString (Msg.GotStripe >> Msg.Nav)
        }
