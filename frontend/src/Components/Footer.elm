module Components.Footer exposing (footer)

import Components.Links exposing (Size(..), links)
import Html exposing (Html)
import Html.Attributes as Attr


footer : () -> Html msg
footer _ =
    Html.footer [ Attr.class "bg-kiggygreen p-4 text-center mt-auto md:hidden" ]
        [ links { size = Large, class = "flex justify-center space-x-4" } ]
