import 'dart:convert';

import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_webrtc/flutter_webrtc.dart';
import 'package:miku/model/game_started.dart';
import 'package:miku/model/ice_candidate_model.dart';
import 'package:miku/provider/game_provider.dart';
import 'package:miku/view/chat_view.dart';
import 'package:provider/provider.dart';
import 'package:web_socket_channel/web_socket_channel.dart';

import '../api/game_manager/request/message_request.dart';
import '../model/chat_model.dart';
import '../model/user_model.dart';
import 'dart:developer' as developer;

class GameView extends StatefulWidget {
  GameView(
      {super.key,
      required this.gameStarted,
      required this.channel,
      required this.user});

  WebSocketChannel channel;
  GameStarted gameStarted;
  User user;

  @override
  _GameViewState createState() => _GameViewState(
        gameStarted: gameStarted,
        channel: channel,
        user: user,
      );
}

class _GameViewState extends State<GameView> {
  _GameViewState(
      {required this.gameStarted, required this.channel, required this.user});

  RTCPeerConnection? _peerConnection;
  MediaStream? _localStream;
  MediaStream? _remoteStream;
  WebSocketChannel channel;
  GameStarted gameStarted;
  User user;
  RTCVideoRenderer _localRenderer = RTCVideoRenderer();
  RTCVideoRenderer _remoteRenderer = RTCVideoRenderer();
  final encoder = JsonEncoder();
  List<ICECandidate> iceCandidates = [];
  final Map<String, dynamic> configuration = {
    'iceServers': [
      {'url': 'stun:stun1.l.google.com:19302'},
      {'url': 'stun:stun2.l.google.com:19302'}
    ],
  };
  final Map<String, dynamic> offerSdpConstraints = {
    "mandatory": {
      "OfferToReceiveAudio": true,
      "OfferToReceiveVideo": true,
    },
    "optional": [],
  };
  bool _isRemoteSet = false;

  @override
  void initState() {
    super.initState();
    //initRenderers();
  }

  initRenderers() async {
    await _remoteRenderer.initialize();
    await _localRenderer.initialize();
    await initPeerConnection();

    if (gameStarted.getHost().id == user.id) {
      await sendOffer();
    }
  }

  sendOffer() async {
    RTCSessionDescription offerSdp = await _peerConnection!.createOffer();
    await _peerConnection?.setLocalDescription(offerSdp);

    String msg = encoder.convert({
      "SDPOffer": {'sdp': offerSdp.sdp}
    });
    channel.sink.add(msg);
  }

  initPeerConnection() async {
    _peerConnection =
        await createPeerConnection(configuration, offerSdpConstraints);

    _localStream = await navigator.mediaDevices.getUserMedia({
      'audio': true,
      'video': true,
    });

    _localStream?.getTracks().forEach((track) {
      _peerConnection?.addTrack(track, _localStream!);
    });
    setState(() {
      _localRenderer.srcObject = _localStream;
    });

    _peerConnection?.onIceCandidate = (RTCIceCandidate candidate) {
      String msg = encoder.convert({
        "RegisterICECandidate": {
          "candidate": candidate.candidate,
          "sdp_mid": candidate.sdpMid,
          "sdp_m_line_index": candidate.sdpMLineIndex
        }
      });
      channel.sink.add(msg);
    };

    _peerConnection?.onAddStream = (stream) {
      print("Stream ajouté");
      setState(() {
        _remoteStream = stream;
        _remoteRenderer.srcObject = _remoteStream;
      });
    };

    _peerConnection?.onTrack = (event) {
      print("Track ajouté");
      setState(() {
        _remoteStream = event.streams[0];
        event.streams[0].getTracks().forEach((track) {
          _remoteStream?.addTrack(track);
        });
        _remoteRenderer.srcObject = _remoteStream;
      });
    };
  }

  answerSdp(GameProvider gameProvider) async {
    print("answerSdp");
    _peerConnection?.setRemoteDescription(
        RTCSessionDescription(gameProvider.offerSDP, "offer"));

    final answerSdp = await _peerConnection?.createAnswer();
    await _peerConnection?.setLocalDescription(answerSdp!);

    String msg = encoder.convert({
      "SDPAnswer": {'sdp': answerSdp!.sdp}
    });
    channel.sink.add(msg);
    _isRemoteSet = true;
  }

  setRemoteAnswer(GameProvider gameProvider) {
    print("setRemoteAnswer");
    _peerConnection?.setRemoteDescription(
        RTCSessionDescription(gameProvider.answerSDP, "answer"));
    _isRemoteSet = true;
  }

  addIceCandidate(GameProvider gameProvider) {
    print("addIceCandidate");
    bool check = false;
    for (ICECandidate iceCandidate in gameProvider.iceCandidates) {
      check = false;
      for (ICECandidate savedIceCandidate in iceCandidates) {
        if (savedIceCandidate.candidate == iceCandidate.candidate &&
            savedIceCandidate.sdp_mid == iceCandidate.sdp_mid &&
            savedIceCandidate.sdp_m_line_index ==
                iceCandidate.sdp_m_line_index) {
          check = true;
        }
      }
      if (!check) {
        _peerConnection!.addCandidate(RTCIceCandidate(iceCandidate.candidate,
            iceCandidate.sdp_mid, iceCandidate.sdp_m_line_index));
        iceCandidates.add(iceCandidate);
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    GameProvider gameProvider = Provider.of<GameProvider>(context);

/*
    if (gameProvider.offerSDP != '' && !_isRemoteSet) {
      answerSdp(gameProvider);
    }

    if (gameProvider.answerSDP != '' && !_isRemoteSet) {
      setRemoteAnswer(gameProvider);
    }

    addIceCandidate(gameProvider);
*/
    if (gameProvider.isShowChat) {
      return WillPopScope(
        onWillPop: () async {
          setState(() {
            gameProvider.isShowChat = false;
          });
          return false;
        },
        child: ChatView(
          messages: gameProvider.messages,
          channel: channel,
          user: user,
        ),
      );
    } else {
      return WillPopScope(
        onWillPop: () async {
          return false;
        },
        child: Scaffold(
          backgroundColor: const Color(0xFF21262B),
          appBar: AppBar(
            automaticallyImplyLeading: false,
            backgroundColor: const Color(0xFF21262B),
            actions: <Widget>[
              Padding(
                padding: const EdgeInsets.only(right: 20.0),
                child: IconButton(
                  icon: const Icon(Icons.chat_bubble),
                  onPressed: () {
                    setState(() {
                      gameProvider.toggleChat(true);
                    });
                  },
                ),
              ),
            ],
          ),
          body: Column(
            children: [
              Expanded(
                child: RTCVideoView(_localRenderer, mirror: true),
              ),
              Expanded(
                child: RTCVideoView(_remoteRenderer),
              ),
            ],
          ),
        ),
      );
    }
  }
}
