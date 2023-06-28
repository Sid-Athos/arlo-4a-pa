import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:miku/model/user_model.dart';
import 'package:provider/provider.dart';
import 'package:web_socket_channel/web_socket_channel.dart';

import '../api/game_manager/request/message_request.dart';
import '../api/game_manager/response/message_response_ws.dart';
import '../provider/game_provider.dart';

class ChatView extends StatefulWidget {
  ChatView({super.key, required this.messages, required this.channel, required this.user});

  List<MessageResponseWS> messages;
  WebSocketChannel channel;
  User user;

  @override
  _ChatViewState createState() => _ChatViewState(
    messages: messages,
    channel: channel,
    user: user,
  );
}

class _ChatViewState  extends State<ChatView> {
  _ChatViewState({required this.messages, required this.channel, required this.user});

  List<MessageResponseWS> messages;
  WebSocketChannel channel;
  User user;

  final ScrollController _scrollController = ScrollController();
  final TextEditingController messageController = TextEditingController();

  @override
  Widget build(BuildContext context) {
    GameProvider gameProvider = Provider.of<GameProvider>(context);

    return Scaffold(
      appBar: AppBar(
          title: const Text("Chat"),
          backgroundColor: const Color(0xFF21262B),
          leading: IconButton(
            icon: const Icon(Icons.arrow_back),
            onPressed: () {
              gameProvider.toggleChat(false);
            },
          ),
      ),
      backgroundColor: const Color(0xFF21262B),
      body: Column(
        children: [
          Expanded(child: buildChat()),
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

  Widget buildChat() {
    return ListView.builder(
      controller: _scrollController,
      itemCount: messages.length,
      itemBuilder: (context, index) {
        return Padding(
          padding: (user.id == messages[index].fromUser.id
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
                      messages[index].fromUser.pseudo,
                      style: const TextStyle(color: Colors.white38),
                    ),
                    subtitle: Text(
                      messages[index].message,
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