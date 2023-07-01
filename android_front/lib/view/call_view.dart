import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:flutter_webrtc/flutter_webrtc.dart';
import 'package:miku/model/user_model.dart';
import 'package:provider/provider.dart';
import 'package:web_socket_channel/web_socket_channel.dart';

import '../api/game_manager/request/message_request.dart';
import '../api/game_manager/response/message_response_ws.dart';
import '../model/rtc_session.dart';
import '../provider/game_provider.dart';

class CallView extends StatefulWidget {
  CallView({super.key, required this.channel, required this.user});

  WebSocketChannel channel;
  User user;

  @override
  _CallViewState createState() => _CallViewState(
        channel: channel,
        user: user,
      );
}

class _CallViewState extends State<CallView> {
  _CallViewState({required this.channel, required this.user});

  WebSocketChannel channel;
  User user;

  @override
  Widget build(BuildContext context) {
    GameProvider gameProvider = Provider.of<GameProvider>(context);

    return Scaffold(
      appBar: AppBar(
        title: const Text("Call"),
        backgroundColor: const Color(0xFF21262B),
        leading: IconButton(
          icon: const Icon(Icons.arrow_back),
          onPressed: () {
            gameProvider.toggleCall(false);
          },
        ),
      ),
      backgroundColor: const Color(0xFF21262B),
      body: (gameProvider.inCall) ? Column(
        children: [
          Expanded(
            child: RTCVideoView(gameProvider.localRenderer, mirror: true),
          ),
          for (RtcSession rtcSession in gameProvider.rtcSessions)
            Expanded(
              child: RTCVideoView(rtcSession.remoteRenderer),
            ),
        ],
      ) : Container(),
      floatingActionButton: (!gameProvider.inCall) ? Padding(
        padding: const EdgeInsets.only(bottom: 25.0),
        child: FloatingActionButton(
          onPressed: gameProvider.joinCall,
          child: Icon(Icons.phone),
          backgroundColor: Colors.green,
        ),
      ) : Row(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          Padding(
            padding: const EdgeInsets.only(bottom: 25.0, right: 25.0),
            child: FloatingActionButton(
              onPressed: () {
                if (cameraCondition(gameProvider)) {
                  gameProvider.toggleLocalVideo(false);
                } else {
                  gameProvider.toggleLocalVideo(true);
                }
              },
              child: Icon(Icons.video_camera_front_rounded),
              backgroundColor: (cameraCondition(gameProvider)) ? Colors.green : Colors.red,
            ),
          ),
          Padding(
            padding: const EdgeInsets.only(bottom: 25.0),
            child: FloatingActionButton(
              onPressed: gameProvider.leaveCall,
              child: Icon(Icons.phone_disabled),
              backgroundColor: Colors.red,
            ),
          ),
          Padding(
            padding: const EdgeInsets.only(bottom: 25.0, left: 25.0),
            child: FloatingActionButton(
              onPressed: () {
                if (audioCondition(gameProvider)) {
                  gameProvider.toggleLocalAudio(false);
                } else {
                  gameProvider.toggleLocalAudio(true);
                }
              },
              child: (audioCondition(gameProvider)) ? Icon(Icons.mic) : Icon(Icons.mic_off),
              backgroundColor: (audioCondition(gameProvider)) ? Colors.green : Colors.red,
            ),
          ),
        ],
      ),
      floatingActionButtonLocation: FloatingActionButtonLocation.centerDocked,
    );
  }

  bool cameraCondition(GameProvider gameProvider) {
    if (gameProvider.localStream == null) {
      return true;
    }
    if (gameProvider.localStream!.getVideoTracks()[0].enabled == true) {
      return true;
    }
    return false;
  }

  bool audioCondition(GameProvider gameProvider) {
    if (gameProvider.localStream == null) {
      return true;
    }
    if (gameProvider.localStream!.getAudioTracks()[0].enabled == true) {
      return true;
    }
    return false;
  }
}
