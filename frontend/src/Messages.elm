module Messages exposing (..)

import Cart exposing (Cart)
import Http
import Stock
import Url exposing (Url)


type Menu
    = Open
    | Closed


type Route
    = Gallery
    | Item Int
    | Checkout
    | About
    | Error


type NavMsg
    = Req Url
    | Change Url
    | GetStripe
    | GotStripe (Result Http.Error String)


type Loading
    = GotCart ( Cart, Stock.StockResult )
    | MakeCart Stock.StockResult


type Msg
    = Load Loading
    | Cart Cart.CartAction
    | FlipMenu
    | Nav NavMsg
    | NoOp
