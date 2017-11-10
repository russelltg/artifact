module Models exposing (..)

import Set
import Dict
import Navigation
import Messages exposing (Route(..))
import Artifacts.Models exposing (..)
import Utils exposing (isJust)


-- CONSTANTS
-- TYPES


{-| user given flags
-}
type alias Flags =
    { readonly : Bool
    , path_url : String
    }


type alias Model =
    { artifacts : Artifacts
    , files : Set.Set String
    , checked : String
    , uuid : String
    , names :
        Dict.Dict NameKey ArtifactId
    , route : Route
    , location : Navigation.Location
    , logs : List LogMsg
    , flags : Flags
    , state : State
    , jsonId : Int
    , create : Maybe EditableArtifact

    -- TODO: there should probably be a revision-id
    -- that has to be preserved to accept renderedText
    , renderedText : Maybe String
    }


{-| current user selections. TODO: store this in a cookie or something...
-}
type alias State =
    { columns : Columns
    , search : Search
    , textView : TextViewState
    }


{-| We can log either a success or a failure to the user
-}
type LogMsg
    = LogOk String
    | LogErr String



-- INIT


initialState : State
initialState =
    { columns = initialColumns
    , search = initialSearch
    , textView = initialTextViewState
    }



-- METHODS


nameIds : Artifacts -> Dict.Dict NameKey ArtifactId
nameIds artifacts =
    let
        pairs =
            List.map (\a -> ( a.name.value, a.id )) (Dict.values artifacts)
    in
        Dict.fromList pairs


getArtifact : NameKey -> Model -> Maybe Artifact
getArtifact name model =
    let
        id =
            Dict.get name model.names
    in
        case id of
            Just id ->
                Dict.get id model.artifacts

            Nothing ->
                Nothing


memberArtifact : NameKey -> Model -> Bool
memberArtifact name model =
    isJust (getArtifact name model)


getCreateArtifact : Model -> EditableArtifact
getCreateArtifact model =
    case model.create of
        Just c ->
            c

        Nothing ->
            { name = ""
            , def = ""
            , text = ""
            , partof = []
            , done = ""
            , revision = 0
            }


{-| get all definition file paths
-}
getFiles : Model -> Maybe String -> List String
getFiles model current =
    let
        files =
            case current of
                Just c ->
                    Set.insert c model.files

                Nothing ->
                    model.files
    in
        Set.toList files
            |> List.sort


{-| get the viewing option for an existing artifact
-}
getEditViewOption : Model -> Artifact -> ViewOption
getEditViewOption model artifact =
    if model.flags.readonly then
        ReadChoice artifact
    else
        case artifact.edited of
            Just e ->
                EditChoice <| ChangeChoice artifact e

            Nothing ->
                ReadChoice <| artifact


{-| The artifact currently being viewed.
-}
type ViewingArtifact
    = ViewingExist ArtifactId
    | ViewingCreate
    | ViewingError String
    | ViewingNothing


{-| Helper function to get the text of artifact that is
currently being viewed.
-}
getViewingArtifact : Model -> ViewingArtifact
getViewingArtifact model =
    case model.route of
        ArtifactNameRoute raw_name ->
            case indexName raw_name of
                Ok name ->
                    case Dict.get name model.names of
                        Just id ->
                            ViewingExist id

                        Nothing ->
                            ViewingNothing

                Err error ->
                    ViewingError <| "invalid artifact name: " ++ error

        ArtifactsRoute ->
            ViewingNothing

        ArtifactEditingRoute ->
            ViewingNothing

        ArtifactCreateRoute ->
            ViewingCreate

        CheckRoute ->
            ViewingNothing

        HelpRoute route ->
            ViewingNothing

        NotFoundRoute ->
            ViewingNothing


{-| Helper function to get the text of the artifact that is
currently being viewed.
-}
getViewingText : Model -> Maybe String
getViewingText model =
    case getViewingArtifact model of
        ViewingExist id ->
            case Dict.get id model.artifacts of
                Nothing ->
                    Nothing

                Just art ->
                    case getEditViewOption model art of
                        ReadChoice art ->
                            Just art.text

                        EditChoice option ->
                            case option of
                                ChangeChoice _ editable ->
                                    Just editable.text

                                CreateChoice editable ->
                                    -- TODO: I think this is impossible
                                    Just editable.text

        ViewingCreate ->
            case model.create of
                Just editable ->
                    Just editable.text

                Nothing ->
                    Nothing

        ViewingError _ ->
            Nothing

        ViewingNothing ->
            Nothing
