import 'dart:convert';

import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_webrtc/flutter_webrtc.dart';
import 'package:miku/model/game_started.dart';
import 'package:miku/model/ice_candidate_model.dart';
import 'package:miku/provider/game_provider.dart';
import 'package:miku/provider/user_ice_candidate_provided.dart';
import 'package:miku/view/chat_view.dart';
import 'package:provider/provider.dart';
import 'package:web_socket_channel/web_socket_channel.dart';

import '../api/game_manager/api_game_manager.dart';
import '../api/game_manager/request/message_request.dart';
import '../model/chat_model.dart';
import '../model/rtc_session.dart';
import '../model/user_model.dart';
import 'dart:developer' as developer;

import '../provider/user_sdp_provided.dart';

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

  WebSocketChannel channel;
  GameStarted gameStarted;
  User user;

  MediaStream? localStream;
  RTCVideoRenderer localRenderer = RTCVideoRenderer();
  List<RtcSession> rtcSessions = [];

  initLocalStream() async {
    await localRenderer.initialize();

    localStream = await navigator.mediaDevices.getUserMedia({
      'audio': true,
      'video': true,
    });

    setState(() {
      localRenderer.srcObject = localStream;
    });
  }
  
  void answerSdpOffer(UserSdp userSdp) async {
    RtcSession rtcSession = RtcSession(userSdp.userId);
    
    rtcSession.initPeerConnection(localStream!, channel);

    await rtcSession.answerSdp(userSdp.sdp, channel);

    rtcSessions.add(rtcSession);
  }

  void setRemoteAnswer(UserSdp userSdp) async {
    for (RtcSession rtcSession in rtcSessions) {
      if (rtcSession.userId == userSdp.userId) {
        rtcSession.setRemoteAnswer(userSdp.sdp);
      }
    }
  }

  void addIceCandidate(UserIceCandidate userIceCandidate) async {
    for (RtcSession rtcSession in rtcSessions) {
      if (rtcSession.userId == userIceCandidate.userId) {
        rtcSession.addIceCandidate(userIceCandidate.iceCandidates);
      }
    }
  }

  void joinCall() async {
    await initLocalStream();

    List<User> usersInCall = await ApiGameManager.joinRtcSession();

    for (User userInCall in usersInCall) {
      RtcSession rtcSession = RtcSession(userInCall.id);

      rtcSession.initPeerConnection(localStream!, channel);
      await rtcSession.sendOffer(channel);

      rtcSessions.add(rtcSession);
    }
  }

  @override
  Widget build(BuildContext context) {
    GameProvider gameProvider = Provider.of<GameProvider>(context);

    for (UserSdp userSdp in gameProvider.offerSDP) {
      if (!userSdp.computed) {
        userSdp.computed = true;
        answerSdpOffer(userSdp);
      }
    }

    for (UserSdp userSdp in gameProvider.answerSDP) {
      if (!userSdp.computed) {
        userSdp.computed = true;
        answerSdpOffer(userSdp);
      }
    }

    for (UserIceCandidate userIceCandidate in gameProvider.iceCandidates) {
      if (!userIceCandidate.computed) {
        userIceCandidate.computed = true;
        addIceCandidate(userIceCandidate);
      }
    }

    if (gameProvider.isShowChat) {
      return WillPopScope(
        onWillPop: () async {
          gameProvider.toggleChat(false);
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
                  icon: const Icon(Icons.call),
                  onPressed: joinCall,
                ),
              ),
              Padding(
                padding: const EdgeInsets.only(right: 20.0),
                child: IconButton(
                  icon: const Icon(Icons.chat_bubble),
                  onPressed: () {
                    gameProvider.toggleChat(true);
                  },
                ),
              ),
            ],
          ),
          body: Column(
            children: [
              Expanded(
                child: RTCVideoView(localRenderer, mirror: true),
              ),
            ],
          ),
        ),
      );
    }
  }
}
