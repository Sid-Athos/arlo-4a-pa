import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:miku/api/user/request/send_friend_request_request.dart';

import '../api/user/api_user.dart';
import '../model/user_model.dart';

class SearchUserView extends StatefulWidget {
  const SearchUserView({super.key});

  @override
  _SearchUserViewState createState() => _SearchUserViewState();
}

class _SearchUserViewState extends State<SearchUserView> {

  final TextEditingController _searchController = TextEditingController();
  Future<List<User>> users = Future(() => []);


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
              users = ApiUser.search(value);
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
                  return UserCardWidget(
                      user: snapshot.data![index]
                  );
                },
              );
            } else if (snapshot.hasError) {
              return Text("${snapshot.error}");
            }
            return const CircularProgressIndicator();
          },
        ),
      )
    );
  }
}

class UserCardWidget extends StatelessWidget {
  UserCardWidget({super.key, required this.user});

  User user;

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
                        onPressed: () {
                          SendFriendRequestRequest sendFriendRequestRequest = SendFriendRequestRequest(
                              recipientId: user.id
                          );
                          ApiUser.sendFriendRequest(sendFriendRequestRequest);
                        },
                        icon: const Icon(
                            Icons.send,
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
