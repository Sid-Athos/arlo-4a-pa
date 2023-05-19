import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';

import '../model/user_model.dart';

class ProfileView extends StatefulWidget {
  ProfileView({super.key, required this.user});

  User user;

  @override
  _ProfileViewState createState() => _ProfileViewState(user: user);
}

class _ProfileViewState extends State<ProfileView> {
  _ProfileViewState({required this.user});

  User user;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: const Color(0xFF21262B),
      body: Padding(
        padding: const EdgeInsets.only(top: 64, left: 16, right: 16),
        child: Column(
          children: <Widget> [
            Row(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                const Icon(
                  Icons.person,
                  color: Colors.white,
                  size: 32,
                ),
                Text(
                  user.pseudo,
                  style: const TextStyle(
                    color: Colors.white,
                    fontSize: 24,
                  ),
                ),
              ],
            ),
            Padding(
              padding: const EdgeInsets.only(top: 32, bottom: 32),
              child: Row(
                children: [
                  const Icon(
                    Icons.email,
                    color: Colors.white,
                    size: 24,
                  ),
                  Padding(
                    padding: const EdgeInsets.only(left: 8),
                    child: Text(
                      user.email,
                      style: const TextStyle(
                        color: Colors.white,
                        fontSize: 16,
                      ),
                    ),
                  ),
                ],
              ),
            ),
            ElevatedButton(
              onPressed: () {},
              style: ElevatedButton.styleFrom(
                backgroundColor: const Color(0xFF626af7),
              ),
              child: Row(
                mainAxisSize: MainAxisSize.min,
                children: const <Widget> [
                  Icon(
                    Icons.edit,
                    color: Colors.white,
                    size: 24,
                  ),
                  Text(
                      "Edit Profile"
                  ),
                ],
              ),
            ),
          ],
        ),
      )
    );
  }
}

