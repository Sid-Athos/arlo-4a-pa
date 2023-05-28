import 'dart:io';

import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:miku/api/user/api_user.dart';
import 'package:shared_preferences/shared_preferences.dart';

import '../model/user_model.dart';

class ProfileOtherView extends StatefulWidget {
  ProfileOtherView({super.key, required this.user});

  User user;

  @override
  _ProfileOtherViewState createState() => _ProfileOtherViewState(user: user);
}

class _ProfileOtherViewState extends State<ProfileOtherView> {
  _ProfileOtherViewState({required this.user});

  User user;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
        appBar: AppBar(
          title: const Text("Profile"),
          backgroundColor: const Color(0xFF21262B),
        ),
        backgroundColor: const Color(0xFF21262B),
        body: Padding(
          padding: const EdgeInsets.only(top: 32, left: 16, right: 16),
          child: Column(
            children: <Widget>[
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
            ],
          ),
        ));
  }
}
