import 'dart:convert';

import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:miku/api/game_manager/api_game_manager.dart';
import 'package:miku/api/game_manager/request/exit_lobby_request.dart';
import 'package:miku/api/game_manager/request/give_host_request.dart';
import 'package:miku/api/game_manager/request/launch_game_request.dart';
import 'package:miku/model/lobby_member_model.dart';
import 'package:miku/view/invite_friend_view.dart';
import 'package:provider/provider.dart';
import 'package:web_socket_channel/web_socket_channel.dart';
import 'dart:developer' as developer;

import '../api/game_manager/request/kick_user_request.dart';
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
                itemCount: (lobby.members.length < lobby.game.maxPlayers
                        ? lobby.members.length + 1
                        : lobby.members.length) +
                    (isHost(lobby.members) ? 1 : 0),
                itemBuilder: (context, index) {
                  return (index < lobby.members.length)
                      ? LobbyMemberCardWidget(
                          lobbyMember: lobby.members[index],
                          channel: channel,
                          user: user,
                          isHost: lobby.getHost().id == user.id,
                        )
                      : (index == lobby.members.length + 2 ||
                              (index == lobby.members.length + 1 &&
                                  isHost(lobby.members)))
                          ? buttonInviteFriend()
                          : buttonStartGame();
                },
              ),
            ),
          ],
        ),
      ),
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

  Widget buttonInviteFriend() {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 8.0, horizontal: 16.0),
      child: ElevatedButton(
        onPressed: () async {
          Navigator.push(
            context,
            MaterialPageRoute(
                builder: (context) => InviteFriendView(
                      channel: channel,
                    )),
          );
        },
        style: ElevatedButton.styleFrom(
          primary: const Color(0xFF1A2025),
          minimumSize: const Size(double.infinity, 64),
          shape: RoundedRectangleBorder(
            borderRadius: BorderRadius.circular(16),
          ),
        ),
        child: const Icon(Icons.add, color: Colors.white),
      ),
    );
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

class LobbyMemberCardWidget extends StatelessWidget {
  LobbyMemberCardWidget(
      {super.key,
      required this.lobbyMember,
      required this.channel,
      required this.user,
      required this.isHost});

  LobbyMember lobbyMember;
  WebSocketChannel channel;
  User user;
  bool isHost;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 8.0, horizontal: 16.0),
      child: Card(
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(16),
        ),
        color: const Color(0xFF1A2025),
        child: Padding(
          padding: const EdgeInsets.only(bottom: 16.0, right: 16.0, left: 16.0),
          child: Column(
            mainAxisSize: MainAxisSize.min,
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: <Widget>[
              Row(
                children: [
                  Flexible(
                    child: ListTile(
                      title: Text(
                        lobbyMember.pseudo,
                        style: const TextStyle(color: Colors.white),
                      ),
                      subtitle: Text(
                        lobbyMember.isHost ? "Host" : "",
                        style: const TextStyle(color: Colors.white38),
                      ),
                    ),
                  ),
                  Row(
                    children: <Widget>[
                      isHost && user.id != lobbyMember.id
                          ? IconButton(
                              onPressed: () {
                                channel.sink.add(
                                    GiveHostRequest.toJson(lobbyMember.id));
                              },
                              icon: const Icon(
                                Icons.diamond,
                                color: Colors.white,
                              ),
                            )
                          : Container(),
                      isHost || user.id == lobbyMember.id
                          ? IconButton(
                              onPressed: () {
                                if (user.id == lobbyMember.id) {
                                  channel.sink.add(ExitLobbyRequest.toJson());
                                  Navigator.pop(context);
                                } else {
                                  channel.sink.add(
                                      KickUserRequest.toJson(lobbyMember.id));
                                }
                              },
                              icon: const Icon(
                                Icons.close,
                                color: Colors.white,
                              ),
                            )
                          : Container(),
                    ],
                  ),
                ],
              ),
            ],
          ),
        ),
      ),
    );
  }
}
