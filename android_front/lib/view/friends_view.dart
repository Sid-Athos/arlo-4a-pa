import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';

class FriendsScreen extends StatefulWidget {
  const FriendsScreen({super.key});

  @override
  _FriendsScreenState createState() => _FriendsScreenState();
}

class _FriendsScreenState extends State<FriendsScreen> {

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

