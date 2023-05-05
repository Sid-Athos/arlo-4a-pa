import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';

class FriendListView extends StatefulWidget {
  const FriendListView({super.key});

  @override
  _FriendListViewState createState() => _FriendListViewState();
}

class _FriendListViewState extends State<FriendListView> {

  @override
  Widget build(BuildContext context) {
    return const Scaffold(
      backgroundColor: Color(0xFF21262B),
      body: Center(
        child: Text("Friends"),
      ),
    );
  }
}

