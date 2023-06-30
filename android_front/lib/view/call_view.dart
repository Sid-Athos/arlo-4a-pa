import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:miku/model/user_model.dart';
import 'package:provider/provider.dart';
import 'package:web_socket_channel/web_socket_channel.dart';

import '../api/game_manager/request/message_request.dart';
import '../api/game_manager/response/message_response_ws.dart';
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

class _CallViewState  extends State<CallView> {
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
      body: Center(
        child:
        Padding(
          padding: const EdgeInsets.only(top: 20),
          child: SizedBox(
            width: double.infinity,
            child: ElevatedButton(
              onPressed: gameProvider.joinCall,
              style: ElevatedButton.styleFrom(
                backgroundColor: const Color(0xFF626af7),
              ),
              child: const Text("Join Call"),
            ),
          ),
        ),
      ),
    );
  }
}