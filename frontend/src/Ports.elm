port module Ports exposing (..)

import Json.Encode exposing (Value)


port setCart : Value -> Cmd msg


port followLink : String -> Cmd msg
