import {onCleanup} from "solid-js";
import {createStore} from "solid-js/store";
import {UserStore} from "../utils/user-store";

export default function useSockets() {

    const [store, setStore] = createStore({
        socket: null,
    });


    const setUpSockets = ()=> {
        const socket = new WebSocket(`ws://localhost:7589/ws?token=${UserStore.get().token}`);

        setStore("socket", socket);

        socket.onopen = async function () {
            //await GamesService.rtcMeeting()
        };
    }

    const sendMessage = (jsonToSend) => {
        store.socket.send(JSON.stringify(jsonToSend))
    }
    onCleanup(() => {
        store.socket?.close;
    });

    store.socket.add

    const handleMessages = (jsonSocketMessage) => {
        if(jsonSocketMessage.Message){
            return jsonSocketMessage.Message;

        }else if(jsonSocketMessage.Emote){
            return jsonSocketMessage.Emote;

        } else if(jsonSocketMessage.UserInvited){
            console.log(jsonSocketMessage.UserInvited)

        }else if(jsonSocketMessage.InviteLobbyCancelled){
            console.log(jsonSocketMessage.InviteLobbyCancelled)

        }else if(jsonSocketMessage.InviteLobbyAccepted){
            console.log(jsonSocketMessage.InviteLobbyAccepted)

        }else if(jsonSocketMessage.InviteLobbyDeclined){
            console.log(jsonSocketMessage.InviteLobbyDeclined)

        }else if(jsonSocketMessage.InviteReceived){
            console.log(jsonSocketMessage.InviteReceived)

        }else if(jsonSocketMessage.GameDisplay){
            console.log(jsonSocketMessage.GameDisplay)

        }else if(jsonSocketMessage.GameAction){
            console.log(jsonSocketMessage.GameAction)

        }else if(jsonSocketMessage.GameStarted){
            console.log(jsonSocketMessage.GameStarted)

        }else if(jsonSocketMessage.ICECandidate){
            console.log(jsonSocketMessage.ICECandidate)

        } else if(jsonSocketMessage.SDPAnswer){
            console.log(jsonSocketMessage.SDPAnswer)

        } else if(jsonSocketMessage.SDPOffer){
            console.log(jsonSocketMessage.SDPOffer)

        } else if(jsonSocketMessage.Lobby){
            console.log(jsonSocketMessage.Lobby)

        } else if(jsonSocketMessage.UserLeftRtcSession){
            console.log(jsonSocketMessage.UserLeftRtcSession)
        } else if(jsonSocketMessage.Error){
            console.log(jsonSocketMessage.Error)
        }else {
            switch(jsonSocketMessage){
                case "Hello":
                    console.log("here")
                    break;
                case "Pong":
                    console.log("here")
                    break;
                case "Bye":
                    console.log("here")
                    break;
                case "LobbyCreated":
                    console.log("here")
                    break;
                case "LobbyJoined":
                    console.log("here")
                    break;
                case "LobbyExited":
                    console.log("here")
                    break;
                case "LobbyHostGiven":
                    console.log("here")
                    break;
                case "LobbyHostTaken":
                    console.log("here")
                    break;
                case "UserJoined":
                    console.log("here")
                    break;
                case "UserKicked":
                    console.log("here")
                    break;
                case "Kicked":
                    console.log("here")
                    break;
                case "UserInvited":
                    console.log("here")
                    break;
                case "StartedGame":
                    console.log("here")
                    break;
                case "CannotStartGame":
                    console.log("here")
                    break;
                case "BadMessage":
                    console.log("here")
                    break;
                case "GameStopped":
                    console.log("here")
                    break;
                case "GameWin":
                    console.log("here")
                    break;
                case "GameLose":
                    console.log("here")
                    break;
                default:
                    alert("Unrecognized message")
            }
        }
    }

    return {
        store,
        setUpSockets,
        sendMessage,
        handleMessages
    };
}