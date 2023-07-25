import {createSignal, onMount} from "solid-js";
import {createStore} from "solid-js/store";
import {UserStore} from "../utils/user-store";
import {createWS} from "@solid-primitives/websocket";
import { createEventSignal } from "@solid-primitives/event-listener";
export default function useSockets() {

    const [socket, setSocket] = createStore({
        websockets:null,
        messageEvent:null
    })
    const [messages, setMessages] = createStore([]);

    const [msg, setMsg] = createSignal({list:[]});
    onMount(() => {
        const ws = createWS(`ws://localhost:7589/ws?token=${UserStore.get().token}`);
        console.log(ws)
        setSocket("websockets",ws)
        sendMessage("Hello")
        ws.addEventListener("message", (ev) => {
                handleMessages(JSON.parse(ev.data))

        });
    })

    const [convo, setConvo] = createSignal([]);



    const sendMessage = (jsonToSend) => {
        socket.websockets?.send(JSON.stringify(jsonToSend))
    }


    const handleMessages = (jsonSocketMessage) => {
        if(jsonSocketMessage.Message){
            setConvo([...convo(),jsonSocketMessage.Message ])
            console.log(convo())
            return jsonSocketMessage.Message;

        }else if(jsonSocketMessage.Emote){
            return jsonSocketMessage.Emote;

        } else if(jsonSocketMessage.UserInvited){
            return jsonSocketMessage.UserInvited

        }else if(jsonSocketMessage.InviteLobbyCancelled){
            return jsonSocketMessage.InviteLobbyCancelled

        }else if(jsonSocketMessage.InviteLobbyAccepted){
            return jsonSocketMessage.InviteLobbyAccepted

        }else if(jsonSocketMessage.InviteLobbyDeclined){
            return jsonSocketMessage.InviteLobbyDeclined

        }else if(jsonSocketMessage.InviteReceived){
            return jsonSocketMessage.InviteReceived

        }else if(jsonSocketMessage.GameDisplay){
            return jsonSocketMessage.GameDisplay

        }else if(jsonSocketMessage.GameAction){
            return jsonSocketMessage.GameAction

        }else if(jsonSocketMessage.GameStarted){
            return jsonSocketMessage.GameStarted

        }else if(jsonSocketMessage.ICECandidate){
            return jsonSocketMessage.ICECandidate

        } else if(jsonSocketMessage.SDPAnswer){
            return jsonSocketMessage.SDPAnswer

        } else if(jsonSocketMessage.SDPOffer){
            return jsonSocketMessage.SDPOffer

        } else if(jsonSocketMessage.Lobby){
            return jsonSocketMessage.Lobby

        } else if(jsonSocketMessage.UserLeftRtcSession){
            return jsonSocketMessage.UserLeftRtcSession
        } else if(jsonSocketMessage.Error){
            return jsonSocketMessage.Error
        }else {
            switch(jsonSocketMessage){
                case "Hello":
                    return jsonSocketMessage
                    break;
                case "Pong":
                    return jsonSocketMessage
                    break;
                case "Bye":
                    return jsonSocketMessage
                    break;
                case "LobbyCreated":
                    return jsonSocketMessage
                    break;
                case "LobbyJoined":
                    return jsonSocketMessage
                    break;
                case "LobbyExited":
                    return jsonSocketMessage
                    break;
                case "LobbyHostGiven":
                    return jsonSocketMessage
                    break;
                case "LobbyHostTaken":
                    return jsonSocketMessage
                    break;
                case "UserJoined":
                    return jsonSocketMessage
                    break;
                case "UserKicked":
                    return jsonSocketMessage
                    break;
                case "Kicked":
                    return jsonSocketMessage
                    break;
                case "UserInvited":
                    return jsonSocketMessage
                    break;
                case "StartedGame":
                    return jsonSocketMessage
                    break;
                case "CannotStartGame":
                    return jsonSocketMessage
                    break;
                case "BadMessage":
                    return jsonSocketMessage
                    break;
                case "GameStopped":
                    return jsonSocketMessage
                    break;
                case "GameWin":
                    return jsonSocketMessage
                    break;
                case "GameLose":
                    return jsonSocketMessage
                    break;
                default:
                    alert("Unrecognized message")
            }
        }
    }

    return {
        socket,
        sendMessage,
        handleMessages,
        msg,
        messages,
        convo,
        setConvo
    };
}