import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:miku/model/game_started.dart';
import 'package:provider/provider.dart';
import 'package:web_socket_channel/web_socket_channel.dart';

import '../api/game_manager/request/message_request.dart';
import '../model/chat_model.dart';
import '../model/user_model.dart';

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
  _GameViewState createState() =>
      _GameViewState(gameStarted: gameStarted, channel: channel, user: user);
}

class _GameViewState extends State<GameView> {
  _GameViewState(
      {required this.gameStarted, required this.channel, required this.user});

  WebSocketChannel channel;
  GameStarted gameStarted;
  User user;
  final ScrollController _scrollController = ScrollController();
  final TextEditingController messageController = TextEditingController();
  bool _isShowChat = false;

  @override
  Widget build(BuildContext context) {
    Chat chat = Provider.of<Chat>(context);

    return WillPopScope(
      onWillPop: () async {
        if (_isShowChat) {
          setState(() {
            _isShowChat = false;
          });
        }
        return false;
      },
      child: _isShowChat ? displayChat(chat) : Scaffold(
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
                    _isShowChat = true;
                  });
                },
              ),
            ),
          ],
        ),
      ),
    );
  }

  Widget displayChat(Chat chat) {
    return Scaffold(
      appBar: AppBar(
        title: const Text("Chat"),
        backgroundColor: const Color(0xFF21262B),
        leading: IconButton(
          icon: const Icon(Icons.arrow_back),
          onPressed: () {
            setState(() {
              _isShowChat = false;
            });
          },
        )
      ),
      backgroundColor: const Color(0xFF21262B),
      body: Column(
        children: [
          Expanded(child: buildChat(chat)),
          TextField(
            controller: messageController,
            style: const TextStyle(color: Colors.white),
            decoration: InputDecoration(
              suffixIcon: IconButton(
                icon: const Icon(
                  Icons.send,
                  color: Colors.white,
                ),
                onPressed: () {
                  send();
                },
              ),
              border: const OutlineInputBorder(),
            ),
            onSubmitted: (String value) {
              channel.sink.add(MessageRequest.toJson(value));
            },
          ),
        ],
      ),
    );
  }

  void send() {
    channel.sink.add(MessageRequest.toJson(messageController.text));
    messageController.clear();
    _scrollController.jumpTo(_scrollController.position.maxScrollExtent);
  }

  Widget buildChat(Chat chat) {
    return ListView.builder(
      controller: _scrollController,
      itemCount: chat.messages.length,
      itemBuilder: (context, index) {
        return Padding(
          padding: (user.id == chat.messages[index].fromUser.id
              ? const EdgeInsets.only(left: 100)
              : const EdgeInsets.only(right: 100)),
          child: Card(
            shape: RoundedRectangleBorder(
              borderRadius: BorderRadius.circular(16),
            ),
            color: const Color(0xFF1A2025),
            child: Padding(
              padding:
              const EdgeInsets.only(bottom: 16.0, right: 32.0, left: 16.0),
              child: Column(
                mainAxisSize: MainAxisSize.min,
                children: <Widget>[
                  ListTile(
                    title: Text(
                      chat.messages[index].fromUser.pseudo,
                      style: const TextStyle(color: Colors.white38),
                    ),
                    subtitle: Text(
                      chat.messages[index].message,
                      style: const TextStyle(color: Colors.white),
                    ),
                  ),
                ],
              ),
            ),
          ),
        );
      },
    );
  }
}
