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
import '../model/rtc_session.dart';
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

  WebSocketChannel channel;
  GameStarted gameStarted;
  User user;

  MediaStream? _localStream;
  RTCVideoRenderer _localRenderer = RTCVideoRenderer();
  List<RtcSession> rtcSessions = [];

  @override
  void initState() {
    super.initState();
  }

  void initLocalStream() async {
    await _localRenderer.initialize();

    _localStream = await navigator.mediaDevices.getUserMedia({
      'audio': true,
      'video': true,
    });

    setState(() {
      _localRenderer.srcObject = _localStream;
    });
  }

  @override
  Widget build(BuildContext context) {
    GameProvider gameProvider = Provider.of<GameProvider>(context);

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
                child: RTCVideoView(_localRenderer, mirror: true),
              ),
            ],
          ),
        ),
      );
    }
  }
}
