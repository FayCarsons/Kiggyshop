module Main exposing (..)

import Browser
import Browser.Navigation as Nav
import Cart exposing (Cart)
import Components.Checkout exposing (checkout)
import Components.Gallery exposing (gallery)
import Components.Icons exposing (Palette(..))
import Components.Loading exposing (errorPage, loadingPage)
import Components.Product exposing (product)
import Dict
import Html exposing (Html, text)
import Http
import Json.Decode as JD
import Json.Encode as JE
import Lib exposing (..)
import Messages exposing (Loading(..), Menu(..), Msg(..), NavMsg(..), Route(..))
import Ports exposing (setCart)
import Stock exposing (Stock)
import Url exposing (Url)
import Url.Parser as Parse exposing ((</>))


type alias Shop =
    { stock : Stock
    , cart : Cart
    }


type alias Page =
    { key : Nav.Key
    , route : Route
    }


type alias AppState =
    { shop : Shop
    , page : Page
    , view : Menu
    }


type Model
    = Uninit Page
    | Failure
    | Success AppState


routeParser : Parse.Parser (Route -> a) a
routeParser =
    Parse.oneOf
        [ Parse.map Gallery Parse.top
        , Parse.map Item (Parse.s "products" </> Parse.int)
        , Parse.s "checkout" </> Parse.map Checkout Parse.top
        , Parse.s "about" </> Parse.map About Parse.top
        ]


parseRoute : Url -> Route
parseRoute url =
    Parse.parse routeParser url |> Maybe.withDefault Gallery


main : Program (Maybe String) Model Msg
main =
    Browser.application { init = init, onUrlChange = onUrlChange, onUrlRequest = onUrlRequest, update = update, view = view, subscriptions = always Sub.none }


init : Maybe String -> Url -> Nav.Key -> ( Model, Cmd Msg )
init maybeCart url key =
    let
        tryCart =
            maybeCart |> Maybe.withDefault "" |> JD.decodeString Cart.cartDecoder
    in
    let
        cmd =
            getStockWithCart (Result.withDefault Dict.empty tryCart)
    in
    ( parseRoute url |> Page key |> Uninit, cmd )


onUrlChange : Url -> Msg
onUrlChange url =
    Change url |> Nav


onUrlRequest : Browser.UrlRequest -> Msg
onUrlRequest req =
    case req of
        Browser.Internal url ->
            Req url |> Nav

        Browser.External _ ->
            NoOp


getStockWithCart : Cart -> Cmd Msg
getStockWithCart jsonCart =
    Http.get
        { url = "/api/stock/get"
        , expect = Http.expectJson (\res -> ( jsonCart, res ) |> GotCart |> Load) Stock.stockDecoder
        }


updateCart : (Int -> Int) -> Int -> AppState -> Maybe AppState
updateCart op itemId ({ shop } as state) =
    findItem itemId shop.stock
        |> Maybe.map
            (\currItem ->
                let
                    newCart =
                        Dict.update itemId (mapCart op currItem) shop.cart
                in
                { state | shop = { shop | cart = newCart } }
            )


matchStateCommand : AppState -> Maybe AppState -> ( Model, Cmd Msg )
matchStateCommand old new =
    case new of
        Just state ->
            ( Success state, setCart (Cart.cartEncoder state.shop.cart) )

        Nothing ->
            ( Success old, Cmd.none )


matchCartAction : Cart.CartAction -> Model -> ( Model, Cmd Msg )
matchCartAction action model =
    case model of
        Success ({ shop } as state) ->
            case action of
                Cart.Inc item ->
                    updateCart inc item state |> matchStateCommand state

                Cart.Dec item ->
                    updateCart dec item state |> matchStateCommand state

                Cart.Remove item ->
                    let
                        newCart =
                            Dict.remove item shop.cart
                    in
                    ( Success { state | shop = { shop | cart = newCart } }, setCart (Cart.cartEncoder newCart) )

                Cart.Clear ->
                    ( Success { state | shop = { shop | cart = Dict.empty } }, setCart JE.null )

        _ ->
            ( model, Cmd.none )


matchLoadAction : Loading -> Page -> ( Model, Cmd Msg )
matchLoadAction msg page =
    case msg of
        GotCart ( cart, maybeStock ) ->
            case maybeStock of
                Ok stock ->
                    ( Success { shop = { stock = stock, cart = cart }, view = Closed, page = page }, Cmd.none )

                Err e ->
                    let
                        _ =
                            throw e
                    in
                    ( Failure, Cmd.none )

        MakeCart maybeStock ->
            case maybeStock of
                Ok stock ->
                    ( Success { shop = { stock = stock, cart = Dict.empty }, view = Closed, page = page }, Cmd.none )

                Err e ->
                    let
                        _ =
                            throw e
                    in
                    ( Failure, Cmd.none )


flipMenu : Menu -> Menu
flipMenu state =
    case state of
        Open ->
            Closed

        Closed ->
            Open


matchMsg : Msg -> Model -> Page -> ( Model, Cmd Msg )
matchMsg msg model routing =
    case msg of
        Load loadMsg ->
            matchLoadAction loadMsg routing

        Cart action ->
            matchCartAction action model

        FlipMenu ->
            case model of
                Success state ->
                    ( Success { state | view = flipMenu state.view }, Cmd.none )

                _ ->
                    ( model, Cmd.none )

        Nav navAction ->
            case model of
                Success ({ page } as state) ->
                    case navAction of
                        Req url ->
                            ( Success state, Nav.pushUrl page.key (Url.toString url) )

                        Change url ->
                            ( Success { state | page = { page | route = parseRoute url } }, Cmd.none )

                        GetStripe ->
                            let
                                _ =
                                    Debug.log "" "Getting stripe url"
                            in
                            ( model, postCheckout state.shop.cart )

                        GotStripe res ->
                            let
                                _ =
                                    Debug.log "Got stripe URL post response" res
                            in
                            case res of
                                Ok s ->
                                    ( model, Nav.load s )

                                Err e ->
                                    let
                                        _ =
                                            Debug.log "Checkout Err" e
                                    in
                                    ( model, Cmd.none )

                _ ->
                    ( model, Cmd.none )

        NoOp ->
            ( model, Cmd.none )


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case model of
        Failure ->
            ( Failure, Cmd.none )

        Success { page } ->
            matchMsg msg model page

        Uninit route ->
            matchMsg msg model route


view : Model -> Browser.Document Msg
view model =
    { title = "KiggyShop"
    , body =
        (case model of
            Failure ->
                errorPage ()

            Success state ->
                matchPage state

            _ ->
                loadingPage ()
        )
            |> List.singleton
    }


matchPage : AppState -> Html Msg
matchPage ({ page } as state) =
    case page.route of
        Gallery ->
            gallery state.shop.stock state.view

        Item id ->
            let
                maybeProduct =
                    Dict.get id state.shop.stock
            in
            case maybeProduct of
                Just item ->
                    product id item state.view

                Nothing ->
                    errorPage ()

        Checkout ->
            checkout state.shop

        About ->
            text "Unimplemented!"

        Error ->
            text "Unimplemented!"
