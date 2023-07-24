import { onCleanup, onMount } from "solid-js";
import { createStore } from "solid-js/store";
import io from "socket.io-client";
import {useNavigate, useParams} from "@solidjs/router";
import {GamesService} from "../utils/services/game-manager-service";
import {UserStore} from "../utils/user-store";

export default function useSockets() {

    const [store, setStore] = createStore({
        socket: null,
    });


    const setUpSockets = ()=> {
        const socket = new WebSocket(`ws://localhost:7589/ws?token=${UserStore.get().token}`);

        setStore("socket", socket);

        socket.onopen = async function () {
            this.send(JSON.stringify({
                CreateLobby: {
                    game_id: 2,
                    private: false
                }
            }));
            await GamesService.rtcMeeting()
        };
        socket.onmessage = function (msg) {
            let res = JSON.parse(msg.data)
            console.log(res)
            if(res.BadMessage){
                return
            }
            this.send("OK")

        };

        socket.addEventListener("sendMessage", (val) => {
            console.log(val)
        })


    }

    onCleanup(() => {
        store.socket?.disconnect();
    });

    const handleMessages = (message)=> {
        return "lol"
    }

    return {
        store,
        setUpSockets
    };
}