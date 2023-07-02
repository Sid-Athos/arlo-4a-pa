import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:miku/api/game_manager/request/exit_lobby_request.dart';
import 'package:miku/api/game_manager/request/launch_game_request.dart';
import 'package:miku/model/lobby_member_model.dart';
import 'package:miku/view/invite_friend_view.dart';
import 'package:miku/view/widget/lobby_card_widget.dart';
import 'package:provider/provider.dart';
import 'package:web_socket_channel/web_socket_channel.dart';

import '../model/lobby_model.dart';
import '../model/user_model.dart';

class LobbyView extends StatefulWidget {
  LobbyView({super.key, required this.channel, required this.user});

  WebSocketChannel channel;
  User user;

  @override
  _LobbyViewState createState() =>
      _LobbyViewState(channel: channel, user: user);
}

class _LobbyViewState extends State<LobbyView> {
  _LobbyViewState({required this.channel, required this.user});

  WebSocketChannel channel;
  User user;

  @override
  Widget build(BuildContext context) {
    final lobby = Provider.of<Lobby>(context);

    return Scaffold(
      appBar: AppBar(
          title: const Text("Lobby"),
          backgroundColor: const Color(0xFF21262B),
          leading: IconButton(
            icon: const Icon(Icons.close),
            onPressed: () {
              channel.sink.add(ExitLobbyRequest.toJson());
              Navigator.pop(context);
            },
          )),
      backgroundColor: const Color(0xFF21262B),
      body: Center(
        child: Column(
          children: <Widget>[
            Expanded(
              child: ListView.builder(
                itemCount: lobby.members.length + (isHost(lobby.members) ? 1 : 0),
                itemBuilder: (context, index) {
                  return (index < lobby.members.length)
                      ? LobbyMemberCardWidget(
                          lobbyMember: lobby.members[index],
                          channel: channel,
                          user: user,
                          isHost: lobby.getHost().id == user.id,
                        )
                      : buttonStartGame();
                },
              ),
            ),
          ],
        ),
      ),
      floatingActionButton: lobby.members.length < lobby.game.maxPlayers ? Padding(
        padding: const EdgeInsets.only(bottom: 25.0),
        child: FloatingActionButton(
          onPressed: () async {
            Navigator.push(
              context,
              MaterialPageRoute(
                  builder: (context) => InviteFriendView(
                    channel: channel,
                  )),
            );
          },
          backgroundColor: const Color(0xFF626af7),
          child: const Icon(Icons.add),
        ),
      ) : null,
    );
  }

  bool isHost(List<LobbyMember> lobbyMembers) {
    for (int i = 0; i < lobbyMembers.length; i++) {
      if (user.id == lobbyMembers[i].id && lobbyMembers[i].isHost) {
        return true;
      }
    }
    return false;
  }

  @override
  void dispose() {
    channel.sink.add(ExitLobbyRequest.toJson());
    super.dispose();
  }

  Widget buttonStartGame() {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 8.0, horizontal: 16.0),
      child: ElevatedButton(
        onPressed: () async {
          channel.sink.add(LaunchGameRequest.toJson());
        },
        style: ElevatedButton.styleFrom(
          primary: const Color(0xFF1A2025),
          minimumSize: const Size(double.infinity, 64),
          shape: RoundedRectangleBorder(
            borderRadius: BorderRadius.circular(16),
          ),
        ),
        child: const Text("Start Game"),
      ),
    );
  }
}
