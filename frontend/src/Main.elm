module Main exposing (..)

import Browser
import Browser.Navigation as Nav
import Cart exposing (Cart, Action(..))
import Components.Checkout exposing (checkout)
import Components.Gallery exposing (gallery)
import Components.Icons exposing (Palette(..))
import Components.Loading exposing (errorPage, loadingPage)
import Components.Product exposing (product)
import Dict
import Html exposing (text)
import Html.Lazy
import Http
import Json.Decode as Decode
import Json.Encode as Encode
import Lib exposing (..)
import Messages exposing (Loading(..), Menu(..), Msg(..), Nav(..), Route(..))
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
            maybeCart |> Maybe.withDefault "" |> Decode.decodeString Cart.decoder
    in
    let
        cmd =
            getStockWithCart (Result.withDefault Dict.empty tryCart)
    in
    ( Uninit (Page key (parseRoute url)), cmd )


onUrlChange : Url -> Msg
onUrlChange url =
    Nav (Change url)


onUrlRequest : Browser.UrlRequest -> Msg
onUrlRequest req =
    case req of
        Browser.Internal url ->
            Nav (Req url)

        Browser.External _ ->
            NoOp


getStockWithCart : Cart -> Cmd Msg
getStockWithCart jsonCart =
    Http.get
        { url = "/api/stock"
        , expect = Http.expectJson (\res -> Load (GotCart ( jsonCart, res ))) Stock.stockDecoder
        }


updateCart : (Int -> Int) -> Int -> AppState -> Maybe AppState
updateCart op itemId ({ shop } as state) =
    Stock.findItem itemId shop.stock
        |> Maybe.map
            (\currItem ->
                let
                    newCart =
                        Dict.update itemId (mapCart op currItem) shop.cart
                in
                { state | shop = { shop | cart = newCart } }
            )


transition : AppState -> Maybe AppState -> ( Model, Cmd Msg )
transition old new =
    case new of
        Just state ->
            ( Success state, setCart (Cart.encoder state.shop.cart) )

        Nothing ->
            ( Success old, Cmd.none )


handleCartAction : Cart.Action -> Model -> ( Model, Cmd Msg )
handleCartAction action model =
    case model of
        Success ({ shop } as state) ->
            case action of
                Inc item ->
                    transition state (updateCart inc item state)

                Dec item ->
                    transition state (updateCart dec item state)

                Remove item ->
                    let
                        newCart =
                            Dict.remove item shop.cart
                    in
                    ( Success { state | shop = { shop | cart = newCart } }, setCart (Cart.encoder newCart) )

                Clear ->
                    ( Success { state | shop = { shop | cart = Dict.empty } }, setCart Encode.null )

        _ ->
            ( model, Cmd.none )


handleLoadAction : Loading -> Page -> ( Model, Cmd Msg )
handleLoadAction msg page =
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


handleMsg : Msg -> Model -> Page -> ( Model, Cmd Msg )
handleMsg msg model routing =
    case msg of
        Load loadMsg ->
            handleLoadAction loadMsg routing

        Cart action ->
            handleCartAction action model

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

                        GotStripe (Ok url) ->
                            let
                                _ =
                                    Debug.log "Got stripe URL post response" url
                            in
                            ( model, Nav.load url )

                        GotStripe (Err e) ->
                            let
                                _ =
                                    Debug.log "Checkout Error" e
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
            ( Failure, Nav.reload )

        Success { page } ->
            handleMsg msg model page

        Uninit route ->
            handleMsg msg model route


view : Model -> Browser.Document Msg
view model =
    { title = "KiggyShop"
    , body =
        (case model of
            Failure ->
                errorPage

            Success state ->
                case state.page.route of
                    Gallery ->
                        Html.Lazy.lazy2 gallery state.shop.stock state.view

                    Item id ->
                        case Dict.get id state.shop.stock of
                            Just item ->
                                Html.Lazy.lazy3 product id item state.view

                            Nothing ->
                                errorPage

                    Checkout ->
                        Html.Lazy.lazy checkout state.shop

                    Error ->
                        text "Unimplemented!"

            _ ->
                loadingPage
        )
            |> List.singleton
    }
