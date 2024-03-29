import 'dart:convert';

import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_svg/svg.dart';
import 'package:flutter_webrtc/flutter_webrtc.dart';
import 'package:miku/api/game_manager/request/game_action_request.dart';
import 'package:miku/model/game/game_started.dart';
import 'package:miku/model/webrtc/ice_candidate_model.dart';
import 'package:miku/provider/game_provider.dart';
import 'package:provider/provider.dart';
import 'package:web_socket_channel/web_socket_channel.dart';

import '../../model/webrtc/rtc_session.dart';
import '../../model/user/user_model.dart';
import 'call_view.dart';
import 'chat_view.dart';

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

  @override
  Widget build(BuildContext context) {
    GameProvider gameProvider = Provider.of<GameProvider>(context);

    if (gameProvider.gameSvgInfo != null)
      print(gameProvider.gameSvgInfo!.createSVG());

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
    } else if (gameProvider.isShowCall) {
      return WillPopScope(
        onWillPop: () async {
          gameProvider.toggleCall(false);
          return false;
        },
        child: CallView(
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
                  onPressed: () {
                    gameProvider.toggleCall(true);
                  },
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
              mainAxisAlignment: MainAxisAlignment.center,
              mainAxisSize: MainAxisSize.max,
              children: [
                Padding(
                  padding: const EdgeInsets.only(bottom: 32),
                  child: Center(
                    child: Text(
                      (gameProvider.gameAction != null) ? "Your turn" : "Opponent turn",
                      style: const TextStyle(color: Colors.white, fontSize: 24),
                    ),
                  ),
                ),
                GestureDetector(
                  onTapUp: (TapUpDetails tapUpDetails) {
                    if (gameProvider.gameAction != null &&
                        gameProvider.gameAction!.isInZones(
                            tapUpDetails.localPosition.dx.floor(),
                            tapUpDetails.localPosition.dy.floor())) {
                      channel.sink.add(GameActionRequest.toJson(
                          tapUpDetails.localPosition.dx.floor(),
                          tapUpDetails.localPosition.dy.floor()));
                      gameProvider.setAction(null);
                    }
                  },
                  child: (gameProvider.gameSvgInfo != null)
                      ? SvgPicture.string(gameProvider.gameSvgInfo!.createSVG())
                      : Container(),
                ),
              ]),
        ),
      );
    }
  }
}
