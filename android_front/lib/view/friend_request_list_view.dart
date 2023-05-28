

import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:miku/model/friend_list_model.dart';

import '../api/user/api_user.dart';
import '../model/user_model.dart';

class FriendRequestListScreen extends StatefulWidget {
  FriendRequestListScreen({super.key, required this.user});

  User user;

  @override
  _FriendRequestListScreenState createState() => _FriendRequestListScreenState(user: user);
}

class _FriendRequestListScreenState extends State<FriendRequestListScreen> {
  _FriendRequestListScreenState({required this.user});

  User user;
  late Future<List<FriendList>> friends;

  @override
  void initState() {
    super.initState();
    friends = ApiUser.getAllUnacceptedRequest(user.id);
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text("Friend Requests"),
        backgroundColor: const Color(0xFF21262B),
      ),
      backgroundColor: const Color(0xFF21262B),
      body: Center(
        child: RefreshIndicator(
          onRefresh: () async {
            setState(() {
              friends = ApiUser.getAllUnacceptedRequest(user.id);
            });
          },
          child: FutureBuilder<List<FriendList>>(
            future: friends,
            builder: (context, snapshot) {
              if (snapshot.hasData && snapshot.data!.length > 0) {
                return ListView.builder(
                  itemCount: snapshot.data?.length,
                  itemBuilder: (context, index) {
                    return snapshot.data![index].recipientId == user.id ? buildFriendRequestCardWidget(snapshot.data![index]): Container();
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
                        Icons.no_accounts,
                        color: Colors.white,
                        size: 48,
                      ),
                    ),
                    Text(
                      "No one wants to be your friend",
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

  Widget buildFriendRequestCardWidget(FriendList friendRequest) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 8.0, horizontal: 16.0),
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
              Row(
                children: [
                  Flexible(
                    child: ListTile(
                      title: Text(
                        friendRequest.applicant?.pseudo ?? "Unknown",
                        style: const TextStyle(color: Colors.white),
                      ),
                    ),
                  ),
                  Row(
                    children: <Widget>[
                      IconButton(
                        onPressed: () {
                          ApiUser.acceptFriendRequest(friendRequest.id);
                          setState(() {
                            friends = ApiUser.getAllUnacceptedRequest(user.id);
                          });
                        },
                        icon: const Icon(
                            Icons.add,
                            color: Colors.white
                        ),
                      ),
                      IconButton(
                        onPressed: () {
                          ApiUser.deleteFriend(friendRequest.applicantId);
                          setState(() {
                            friends = ApiUser.getAllUnacceptedRequest(user.id);
                          });
                        },
                        icon: const Icon(
                            Icons.close,
                            color: Colors.white
                        ),
                      ),
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