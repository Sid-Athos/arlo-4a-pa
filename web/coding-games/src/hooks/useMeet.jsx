import { onCleanup, onMount } from "solid-js";
import { createStore } from "solid-js/store";
import io from "socket.io-client";
import {useNavigate, useParams} from "@solidjs/router";
import {ApiInstance} from "../utils/services/api-instance";
import {GameManagerInstance} from "../utils/services/game-manager-instance";
import {GamesService} from "../utils/services/game-manager-service";
export default function useMeet() {
    const navigate = useNavigate();
    const params = useParams();

    const [store, setStore] = createStore({
        error: null,
        socket: null,
        peer: null,
        currentStream: null,
        currentUser: null,

        remoteStream: null,
        remoteUser: null,
        remoteMuted: false,
        remoteWebCam: true,

        incomingCall: false,
        incomingPayload: null,

        muted: false,
        webCam: true,
    });

    onMount(async () => {
        await requestMediaAccess();
    });

    onMount(async () => {
        console.log("http:///localhost:7589/rtc/join_rtc")
        const socket = io("http:///localhost:7589?");

        setStore("socket", socket);

        socket.emit("joinRoom", params.meetCode);

        socket.on("currentUser", (user) => {
            setStore("currentUser", user);
        });
        socket.on("userJoined", (user) => {
            console.log(user)
            setStore("remoteUser", user);
            callUser(user);
        });
        socket.on("remoteUsers", (users) => {
            setStore("remoteUser", users.length ? users[0] : null);
        });

        socket.on("userDisconnected", (user) => {
            setStore("remoteUser", null);
        });

        store.socket.on("offer", (data) => {
            console.info("incomming remote offer:-", data)
            setStore("incommingCall", true);
            setStore("incommingPayload", data);
        });

        store.socket.on("answer", handleAnswer);

        store.socket.on("ice-candidate", handleNewICECandidateMsg);

        store.socket.on("remote-mic-toggle", (payload) => {
            setStore("remoteMuted", payload.muted);
        });
        store.socket.on("remote-cam-toggle", (payload) => {
            setStore("remoteWebCam", payload.webCam);
        });

        socket.on("call-end", () => {
            endCall();
        });
    });

    onCleanup(() => {
        store.socket?.disconnect();
    });

    async function requestMediaAccess() {
        try {
            let stream;
            if((await navigator.mediaDevices.enumerateDevices()).find(device => device.kind === "videoinput")){
                stream = await navigator.mediaDevices.getUserMedia({
                    audio: true,
                    video: {
                        width: 640,
                        height: 480,
                    },
                });
            } else {
                stream = await navigator.mediaDevices.getUserMedia({
                    audio: true
                });
            }


            setStore("currentStream", stream);

            if (store.incommingCall) {
                handleRecieveCall(store.incommingPayload);
            }
        } catch (error) {
            console.error("media access permission error:-", error);
            setStore("error", { name: error.name, message: error.message });
        }
    }

    function createPeer(socketId) {
        const peer = new RTCPeerConnection({
            iceServers: [
                {
                    urls: "stun:numb.viagenie.ca",
                },
                {
                    urls: "turn:numb.viagenie.ca",
                    username: "harshdev8218@gmail.com",
                    credentialType: "password",
                    credential: "numbDev@2022",
                },
            ],
        });
        setStore("peer", peer);
        peer.onicecandidate = handleICECandidateEvent;
        peer.ontrack = handleTrackEvent;
        peer.onnegotiationneeded = () => handleNegotiationNeededEvent(socketId);
        return peer;
    }

    function callUser(soketId) {
        store.peer = createPeer(soketId);
        store.currentStream
            .getTracks()
            .forEach((track) => store.peer.addTrack(track, store.currentStream));
    }

    function handleNegotiationNeededEvent(socketId) {
        store.peer
            .createOffer()
            .then((offer) => {
                console.info("offer  created successfully.");
                return store.peer.setLocalDescription(offer);
            })
            .then(() => {
                const payload = {
                    to: socketId,
                    from: store.currentUser,
                    sdp: store.peer.localDescription,
                };
                store.socket.emit("offer", payload);
            })
            .catch((e) => {
                console.error("create offer error:-", e);
            });
    }

    function handleRecieveCall(data) {
        const peer = createPeer();
        const desc = new RTCSessionDescription(data.sdp);
        peer
            .setRemoteDescription(desc)
            .then(() => {
                store.currentStream
                    ?.getTracks()
                    .forEach((track) => peer.addTrack(track, store.currentStream));
            })
            .then(() => {
                console.info("create answer")
                return peer.createAnswer();
            })
            .then((answer) => {
                console.info("answer created successfully:-",answer)
                return peer.setLocalDescription(answer);
            })
            .then(() => {
                const payload = {
                    to: data.from,
                    from: store.currentUser,
                    sdp: peer.localDescription,
                };
                store.socket.emit("answer", payload);
                console.info("sent answer to remote.")
            });
    }

    function handleAnswer(data) {
        console.log("remote answer:-", data);
        const desc = new RTCSessionDescription(data.sdp);
        store.peer.setRemoteDescription(desc).catch((e) => console.log(e));
    }

    function handleICECandidateEvent(e) {
        console.info("gathering your ice candidate")
        if (e.candidate) {
            console.log("your ice-candidate:-", e.candidate);
            const payload = {
                to: store.remoteUser,
                candidate: e.candidate,
            };
            store.socket.emit("ice-candidate", payload);
        }
    }

    function handleNewICECandidateMsg(data) {
        console.log("incomming remote ice-candidate:-", data.candidate);
        const candidate = new RTCIceCandidate(data.candidate);
        store.peer?.addIceCandidate(candidate).catch((e) => console.log(e));
    }

    function handleTrackEvent(e) {
        console.info("incomming remote stream", e.streams);
        setStore("remoteStream", e.streams[0]);
    }

    function toggleMic() {
        setStore("muted", !store.muted);
        store.currentStream.getAudioTracks()[0].enabled =
            !store.currentStream.getAudioTracks()[0].enabled;

        store.socket.emit("remote-mic-toggle", {
            to: store.remoteUser,
            from: store.currentUser,
            muted: store.muted,
        });
    }

    function toggleWebCam() {
        setStore("webCam", !store.webCam);
        store.currentStream.getVideoTracks()[0].enabled =
            !store.currentStream.getVideoTracks()[0].enabled;

        store.socket.emit("remote-cam-toggle", {
            to: store.remoteUser,
            from: store.currentUser,
            webCam: store.webCam,
        });
    }

    function cleanUserMediaStream() {
        if (store.currentStream) {
            store.currentStream.getTracks().forEach((track) => track.stop());
        }
    }

    function endCall() {
        cleanUserMediaStream();
        store.socket.emit("call-end", {
            from: store.currentUser,
            to: store.remoteUser,
        });
        navigate("/", { replace: true });
    }

    return {
        store,
        toggleMic,
        toggleWebCam,
        endCall,
    };
}