import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:miku/api/game_manager/request/message_request.dart';
import 'package:miku/model/chat_model.dart';
import 'package:miku/model/user_model.dart';
import 'package:provider/provider.dart';
import 'package:web_socket_channel/web_socket_channel.dart';

class ChatView extends StatefulWidget {
  ChatView({super.key, required this.channel, required this.user});

  WebSocketChannel channel;
  User user;

  @override
  _ChatViewState createState() => _ChatViewState(channel: channel, user: user);
}

class _ChatViewState extends State<ChatView> {
  _ChatViewState({required this.channel, required this.user});

  WebSocketChannel channel;
  User user;

  @override
  Widget build(BuildContext context) {
    Chat chat = Provider.of<Chat>(context);

    return Scaffold(
      appBar: AppBar(
          title: const Text("Chat"),
          backgroundColor: const Color(0xFF21262B),
      ),
      backgroundColor: const Color(0xFF21262B),
      body: Column(
        children: [
          buildChat(chat),
          TextField(
            decoration: const InputDecoration(
              suffixIcon: Icon(Icons.send),
              border: OutlineInputBorder(),
            ),
            onSubmitted: (String value) {
              channel.sink.add(MessageRequest.toJson(value));
            },
          ),
        ],
      ),
    );
  }

  Widget buildChat(Chat chat) {
    return ListView.builder(
      itemCount: chat.messages.length,
      itemBuilder: (context, index) {
        return Padding(
          padding: (user.id == chat.messages[index].fromUser.id
              ? const EdgeInsets.only(left: 100)
              : const EdgeInsets.only(right: 100)),
          child: ListTile(
            title: Text(chat.messages[index].message),
            subtitle: Text(chat.messages[index].fromUser.pseudo),
          ),
        );
      },
    );
  }
}
