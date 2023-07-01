import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:miku/api/user/request/send_friend_request_request.dart';
import 'package:miku/model/friend_list_model.dart';
import 'package:miku/view/profile_other_view.dart';
import 'dart:developer' as developer;

import '../api/user/api_user.dart';
import '../model/user_model.dart';

class SearchUserView extends StatefulWidget {
  SearchUserView({super.key, required this.user});

  User user;

  @override
  _SearchUserViewState createState() => _SearchUserViewState(user: user);
}

class _SearchUserViewState extends State<SearchUserView> {
  _SearchUserViewState({required this.user});

  User user;
  List<FriendList> sentRequest = [];
  final TextEditingController _searchController = TextEditingController();
  Future<List<User>> users = Future(() => []);

  Future<List<User>> search(String search) async {
    List<FriendList> tmp = await ApiUser.getAllUnacceptedRequestWithApplicant(user.id);
    setState(() {
      sentRequest = tmp;
    });
    List<User> users = await ApiUser.search(search);
    List<FriendList> alreadyFriends = await ApiUser.getFriendList();
    List<User> result = [];
    for (User userReceived in users) {
      bool found = false;
      for (FriendList friendList in alreadyFriends) {
        if (userReceived.id == friendList.applicantId ||
            userReceived.id == friendList.recipientId) {
          found = true;
          break;
        }
      }
      if (user.id == userReceived.id) {
        found = true;
      }
      if (!found) {
        result.add(userReceived);
      }
    }
    return result;
  }

  @override
  void initState() {
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
        appBar: AppBar(
          automaticallyImplyLeading: false,
          title: TextField(
            controller: _searchController,
            style: const TextStyle(color: Colors.white),
            cursorColor: Colors.white,
            decoration: const InputDecoration(
              hintText: 'Search...',
              hintStyle: TextStyle(color: Colors.white54),
              border: InputBorder.none,
            ),
            onChanged: (value) {
              setState(() {
                users = search(value);
              });
            },
          ),
          backgroundColor: const Color(0xFF21262B),
          actions: <Widget>[
            Padding(
                padding: const EdgeInsets.only(right: 20.0),
                child: GestureDetector(
                  onTap: () {
                    Navigator.pop(context);
                  },
                  child: const Icon(
                    Icons.close,
                    size: 32.0,
                  ),
                )),
          ],
        ),
        backgroundColor: const Color(0xFF21262B),
        body: Center(
          child: FutureBuilder<List<User>>(
            future: users,
            builder: (context, snapshot) {
              if (snapshot.hasData && snapshot.data!.isNotEmpty) {
                return ListView.builder(
                  itemCount: snapshot.data?.length,
                  itemBuilder: (context, index) {
                    return userCardWidget(snapshot.data![index]);
                  },
                );
              } else if (snapshot.hasError) {
                return Text("${snapshot.error}");
              }
              return const CircularProgressIndicator();
            },
          ),
        ));
  }

  bool isRequested(User friendRequestUser) {
    for (FriendList request in sentRequest) {
      if (request.recipientId == friendRequestUser.id) {
        return true;
      }
    }
    return false;
  }

  FriendList? getRequested(User friendRequestUser) {
    for (FriendList request in sentRequest) {
      if (request.recipientId == friendRequestUser.id) {
        return request;
      }
    }
    return null;
  }

  Widget userCardWidget(User friendRequestUser) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 8.0, horizontal: 16.0),
      child: InkWell(
        onTap: () {
          Navigator.push(
            context,
            MaterialPageRoute(
                builder: (context) => ProfileOtherView(user: friendRequestUser)),
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
                          friendRequestUser.pseudo,
                          style: const TextStyle(color: Colors.white),
                        ),
                      ),
                    ),
                    Row(
                      children: <Widget>[
                        isRequested(friendRequestUser) ? IconButton(
                          onPressed: () async {
                            await ApiUser.deleteFriend(getRequested(friendRequestUser)!.id);
                            users = search(_searchController.text);
                          },
                          icon: const Icon(Icons.cancel_schedule_send, color: Colors.white),
                        ): IconButton(
                          onPressed: () async {
                            SendFriendRequestRequest sendFriendRequestRequest =
                            SendFriendRequestRequest(recipientId: friendRequestUser.id);
                            await ApiUser.sendFriendRequest(sendFriendRequestRequest);
                            users = search(_searchController.text);
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
