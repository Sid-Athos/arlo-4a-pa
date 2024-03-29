import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:miku/view/login/subscribe_view.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:web_socket_channel/web_socket_channel.dart';
import 'dart:developer' as developer;

import '../../api/game_manager/api_game_manager.dart';
import '../../api/user/api_user.dart';
import '../../api/user/request/login_request.dart';
import '../../model/user/user_model.dart';
import '../home_view.dart';

class LoginPage extends StatefulWidget {
  const LoginPage({super.key});

  @override
  State<LoginPage> createState() => _LoginPageState();
}

class _LoginPageState extends State<LoginPage> {
  final GlobalKey<FormState> _formKey = GlobalKey<FormState>();
  final emailController = TextEditingController();
  final passwordController = TextEditingController();
  String error = "";

  void signUserIn() async {
    if (_formKey.currentState!.validate()) {
      LoginRequest loginRequest = LoginRequest(
          nickname: emailController.text,
          password: passwordController.text
      );
      final session = await ApiUser.login(loginRequest);
      if (session != null) {
        SharedPreferences prefs = await SharedPreferences.getInstance();
        prefs.setString("login_token", session.token);
        WebSocketChannel? channel = ApiGameManager.openWebSocketConnection(prefs.getString('login_token')!);
        if (channel == null) {
          prefs.remove('login_token');
          wrongCredentials();
          return;
        }
        User user = (await ApiUser.me(prefs.getString('login_token')!))!;
        Navigator.pop(context);
        Navigator.push(
          context,
          MaterialPageRoute(builder: (context) => HomeView(channel: channel!, user: user)),
        );
      } else {
        wrongCredentials();
      }
    }
  }

  void wrongCredentials() {
    setState(() {
      error = "Wrong credentials";
    });
  }

  String? _textEmptyValidator(String? value) {
    if (value == null || value.isEmpty) {
      return 'Please, fill this input';
    }
    return null;
  }

  @override
  Widget build(BuildContext context) {
    SystemChrome.setPreferredOrientations([
      DeviceOrientation.portraitUp,
      DeviceOrientation.portraitDown,
    ]);
    return Scaffold(
      body: Center(
        child: SingleChildScrollView(
          child: Container(
            width: MediaQuery.of(context).size.width,
            height: MediaQuery.of(context).size.height,
            decoration: const BoxDecoration(
              image: DecorationImage(
                image: AssetImage("assets/img/background.png"),
                fit: BoxFit.cover,
              ),
            ),
            child: Column(
              mainAxisAlignment: MainAxisAlignment.center,
              children: <Widget>[
                const Padding(
                  padding: EdgeInsets.symmetric(horizontal: 40, vertical: 16),
                  child: Text(
                    "Login",
                    style: TextStyle(
                      fontFamily: 'GoogleSans',
                      fontSize: 40,
                      color: Colors.white,
                    ),
                  ),
                ),
                Padding(
                  padding: const EdgeInsets.only(left: 40, right: 40, top: 10),
                  child: Form(
                    key: _formKey,
                    child: Column(
                      children: <Widget>[
                        TextFormField(
                          validator: _textEmptyValidator,
                          controller: emailController,
                          decoration: const InputDecoration(
                            enabledBorder: UnderlineInputBorder(
                              borderSide: BorderSide(color: Colors.white),
                            ),
                            labelText: 'Enter your nickname',
                            labelStyle: TextStyle(color: Colors.white),
                          ),
                          style: const TextStyle(color: Colors.white),
                        ),
                        TextFormField(
                          validator: _textEmptyValidator,
                          controller: passwordController,
                          decoration: const InputDecoration(
                            enabledBorder: UnderlineInputBorder(
                              borderSide: BorderSide(color: Colors.white),
                            ),
                            labelText: 'Enter your password',
                            labelStyle: TextStyle(color: Colors.white),
                          ),
                          obscureText: true,
                          style: const TextStyle(color: Colors.white),
                        ),
                        Padding(
                          padding: const EdgeInsets.only(top: 20),
                          child: Text(error,
                              style: const TextStyle(color: Colors.red)
                          ),
                        ),
                        Padding(
                          padding: const EdgeInsets.only(top: 20),
                          child: SizedBox(
                            width: double.infinity,
                            child: ElevatedButton(
                              onPressed: signUserIn,
                              style: ElevatedButton.styleFrom(
                                backgroundColor: const Color(0xFF626af7),
                              ),
                              child: const Text("Log In"),
                            ),
                          ),
                        ),
                      ],
                    ),
                  ),
                ),
                Padding(
                  padding: const EdgeInsets.only(top: 40),
                  child: GestureDetector(
                    onTap: () {
                      Navigator.push(
                        context,
                        MaterialPageRoute(
                            builder: (context) => const SubscribeView()),
                      );
                    },
                    child: const Text(
                      "Create an account",
                      style: TextStyle(
                        color: Colors.grey,
                        decoration: TextDecoration.underline,
                      ),
                    ),
                  ),
                ),
                const SizedBox(height: 50),
              ],
            ),
          ),
        ),
      ),
    );
  }
}
