import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:miku/api/user/api_user.dart';
import 'package:miku/model/friend_list_model.dart';
import 'package:miku/model/user_model.dart';
import 'package:miku/view/friend_request_list_view.dart';
import 'package:miku/view/profile_other_view.dart';
import 'package:miku/view/search_user_view.dart';

class FriendListView extends StatefulWidget {
  FriendListView({super.key, required this.user});

  User user;

  @override
  _FriendListViewState createState() => _FriendListViewState(user: user);
}

class _FriendListViewState extends State<FriendListView> {
  _FriendListViewState({required this.user});

  User user;
  late Future<List<FriendList>> users;

  @override
  void initState() {
    super.initState();
    users = ApiUser.getFriendList();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        automaticallyImplyLeading: false,
        title: const Text("Friends"),
        backgroundColor: const Color(0xFF21262B),
        actions: <Widget>[
          Padding(
              padding: const EdgeInsets.only(right: 20.0),
              child: GestureDetector(
                onTap: () {
                  Navigator.push(
                    context,
                    MaterialPageRoute(
                        builder: (context) => SearchUserView(user: user)),
                  ).then((_) => setState(() {
                        users = ApiUser.getFriendList();
                      }));
                },
                child: const Icon(
                  Icons.search,
                  size: 32.0,
                ),
              )),
          Padding(
              padding: const EdgeInsets.only(right: 20.0),
              child: GestureDetector(
                onTap: () {
                  Navigator.push(
                    context,
                    MaterialPageRoute(
                        builder: (context) =>
                            FriendRequestListScreen(user: user)),
                  ).then((_) => setState(() {
                        users = ApiUser.getFriendList();
                      }));
                },
                child: const Icon(
                  Icons.mail,
                  size: 32.0,
                ),
              )),
        ],
      ),
      backgroundColor: const Color(0xFF21262B),
      body: Center(
        child: RefreshIndicator(
          onRefresh: () async {
            setState(() {
              users = ApiUser.getFriendList();
            });
          },
          child: FutureBuilder<List<FriendList>>(
            future: users,
            builder: (context, snapshot) {
              if (snapshot.hasData && snapshot.data!.length > 0) {
                return ListView.builder(
                  itemCount: snapshot.data?.length,
                  itemBuilder: (context, index) {
                    return buildFriendCardWidget(snapshot.data![index]);
                  },
                );
              } else if (snapshot.hasError) {
                return Text("${snapshot.error}");
              } else if (snapshot.hasData && snapshot.data!.length == 0) {
                return SingleChildScrollView(
                  physics: const AlwaysScrollableScrollPhysics(),
                  child: Column(
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
                        "No Friends",
                        style: TextStyle(
                          color: Colors.white,
                          fontSize: 16,
                        ),
                      ),
                    ],
                  ),
                );
              }
              return CircularProgressIndicator();
            },
          ),
        ),
      ),
    );
  }

  Widget buildFriendCardWidget(FriendList friendList) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 8.0, horizontal: 16.0),
      child: InkWell(
        onTap: () {
          Navigator.push(
            context,
            MaterialPageRoute(
                builder: (context) => ProfileOtherView(
                    user: friendList.applicantId == user.id
                        ? friendList.recipient
                        : friendList.applicant)),
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
                          friendList.applicantId == user.id
                              ? friendList.recipient.pseudo
                              : friendList.applicant.pseudo,
                          style: const TextStyle(color: Colors.white),
                        ),
                      ),
                    ),
                    Row(
                      children: <Widget>[
                        IconButton(
                          onPressed: () async {
                            await ApiUser.deleteFriend(friendList.id);
                            setState(() {
                              users = ApiUser.getFriendList();
                            });
                          },
                          icon: const Icon(Icons.close, color: Colors.white),
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
