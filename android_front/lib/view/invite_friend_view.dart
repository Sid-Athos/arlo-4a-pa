import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:miku/api/game_manager/request/join_lobby_request.dart';
import 'package:miku/model/game_model.dart';
import 'package:miku/model/lobby_model.dart';
import 'package:miku/view/profile_other_view.dart';
import 'package:web_socket_channel/web_socket_channel.dart';

import '../api/game_manager/api_game_manager.dart';
import '../api/game_manager/request/create_lobby_request.dart';
import '../api/game_manager/request/invite_lobby_request.dart';
import '../model/user_model.dart';

class InviteFriendView extends StatefulWidget {
  InviteFriendView({super.key, required this.channel});

  WebSocketChannel channel;

  @override
  _InviteFriendViewState createState() => _InviteFriendViewState(channel: channel);
}

class _InviteFriendViewState extends State<InviteFriendView> {
  _InviteFriendViewState({required this.channel});

  late Future<List<User>> users;
  WebSocketChannel channel;

  @override
  void initState() {
    super.initState();
    users = ApiGameManager.getConnectedFriends();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text("Invite Friend"),
        backgroundColor: const Color(0xFF21262B),
      ),
      backgroundColor: const Color(0xFF21262B),
      body: Center(
        child: RefreshIndicator(
          onRefresh: () async {
            setState(() {
              users = ApiGameManager.getConnectedFriends();
            });
          },
          child: FutureBuilder<List<User>>(
            future: users,
            builder: (context, snapshot) {
              if (snapshot.hasData && snapshot.data!.length > 0) {
                return ListView.builder(
                  itemCount: snapshot.data?.length,
                  itemBuilder: (context, index) {
                    return userCardWidget(snapshot.data![index]);
                  },
                );
              } else if (snapshot.hasError) {
                return Text("${snapshot.error}");
              } else if (snapshot.hasData && snapshot.data!.length == 0) {
                return Column(
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: const <Widget>[
                    Padding(
                      padding: EdgeInsets.all(16.0),
                      child: Icon(
                        Icons.search_off,
                        color: Colors.white,
                        size: 48,
                      ),
                    ),
                    Text(
                      "No Friends Connected",
                      style: TextStyle(
                        color: Colors.white,
                        fontSize: 16,
                      ),
                    ),
                  ],
                );
              }
              return CircularProgressIndicator();
            },
          ),
        ),
      ),
    );
  }

  Widget userCardWidget(User user) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 8.0, horizontal: 16.0),
      child: InkWell(
        onTap: () {
          Navigator.push(
            context,
            MaterialPageRoute(
                builder: (context) => ProfileOtherView(user: user)),
          );
        },
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
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: <Widget>[
                Row(
                  children: [
                    Flexible(
                      child: ListTile(
                        title: Text(
                          user.pseudo,
                          style: const TextStyle(color: Colors.white),
                        ),
                      ),
                    ),
                    Row(
                      children: <Widget>[
                        IconButton(
                          onPressed: () async {
                            channel.sink.add(
                                InviteLobbyRequest.toJson(user.id)
                            );
                          },
                          icon: const Icon(Icons.send, color: Colors.white),
                        ),
                      ],
                    ),
                  ],
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}
