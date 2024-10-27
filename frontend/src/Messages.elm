module Messages exposing (..)

import Cart exposing (Cart)
import Http
import Stock exposing (Stock)
import Url exposing (Url)


type Menu
    = Open
    | Closed


type Route
    = Gallery
    | Item Int
    | Checkout
    | Error


type Nav
    = Req Url
    | Change Url
    | GetStripe
    | GotStripe (Result Http.Error String)


type Loading
    = GotCart ( Cart, Result Http.Error Stock )
    | MakeCart (Result Http.Error Stock)


type Msg
    = Load Loading
    | Cart Cart.Action
    | FlipMenu
    | Nav Nav
    | NoOp
