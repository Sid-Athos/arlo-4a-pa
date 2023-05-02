import 'package:flutter/material.dart';
import 'dart:developer' as developer;

import 'package:flutter/services.dart';
import 'package:miku/api/user/request/create_user_request.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:web_socket_channel/web_socket_channel.dart';

import '../api/game_manager/api_game_manager.dart';
import '../api/game_manager/response/response_enum_ws.dart';
import '../api/user/api_user.dart';
import '../api/user/request/login_request.dart';
import '../model/user_model.dart';
import 'home_view.dart';

class SubscribeView extends StatefulWidget {
  static String routeName = "Subscribe";

  const SubscribeView({Key? key}) : super(key: key);

  @override
  State<SubscribeView> createState() => _SubscribeState();
}

class _SubscribeState extends State<SubscribeView> {
  final GlobalKey<FormState> _formKey = GlobalKey<FormState>();
  final pseudoController = TextEditingController();
  final emailController = TextEditingController();
  final passwordController = TextEditingController();
  final passwordConfirmController = TextEditingController();
  String error = "";

  void subscribe() async {
    if (_formKey.currentState!.validate()) {
      CreateUserRequest createUserRequest = CreateUserRequest(
          nickname: pseudoController.text,
          email: emailController.text,
          password: passwordController.text
      );
      User? user = await ApiUser.createUser(createUserRequest);

      if (user != null) {
        LoginRequest loginRequest = LoginRequest(
            nickname: pseudoController.text,
            password: passwordController.text
        );
        final session = await ApiUser.login(loginRequest);
        if (session != null) {
          SharedPreferences prefs = await SharedPreferences.getInstance();
          WebSocketChannel channel = ApiGameManager.openWebSocketConnection(prefs.getString('login_token')!);
          channel.stream.listen((message)
            {
              ResponseWS.computeResponse(message);
            }
          );
          Navigator.pop(context);
          Navigator.push(
            context,
            MaterialPageRoute(builder: (context) => HomeView(channel: channel)),
          );
        } else {
          wrongCredentials();
        }
      } else {
        wrongCredentials();
      }
    }
  }

  void wrongCredentials() {
    setState(() {
      error = "Email already exist";
    });
  }

  String? _textEmptyValidator(String? value) {
    if (value == null || value.isEmpty) {
      return 'Please, fill this input';
    }
    return null;
  }

  String? _passwordValidator(String? value) {
    if (value == null || value.isEmpty) {
      return 'Please, fill this input';
    }
    if (value.length < 8) {
      return 'Password must contain at least 8 characters';
    }
    const pattern = r'^(?=.*?[A-Z])(?=.*?[0-9])(?=.*?[!@#\$&*~]).{8,}$';
    final regExp = RegExp(pattern);
    if (!regExp.hasMatch(value)) {
      return 'Rules : 1 capital letter, 1 digit, 1 special character';
    }
    return null;
  }

  String? _passwordConfirmValidator(String? value) {
    if (value == null || value.isEmpty) {
      return 'Please, fill this input';
    }
    if (passwordController.text != value) {
      return 'Must be the same that the password';
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
                  fit: BoxFit.cover),
            ),
            child: Column(
              mainAxisAlignment: MainAxisAlignment.center,
              children: <Widget>[
                const Padding(
                  padding: EdgeInsets.symmetric(horizontal: 40, vertical: 0),
                  child: Text(
                    "Subscribe",
                    style: TextStyle(
                        fontFamily: 'GoogleSans',
                        fontSize: 40,
                        color: Colors.white),
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
                          controller: pseudoController,
                          decoration: const InputDecoration(
                            enabledBorder: UnderlineInputBorder(
                              borderSide: BorderSide(color: Colors.white),
                            ),
                            labelText: 'Enter your pseudo',
                            labelStyle: TextStyle(color: Colors.white),
                          ),
                          style: const TextStyle(color: Colors.white),
                        ),
                        TextFormField(
                          validator: _textEmptyValidator,
                          controller: emailController,
                          decoration: const InputDecoration(
                            enabledBorder: UnderlineInputBorder(
                              borderSide: BorderSide(color: Colors.white),
                            ),
                            labelText: 'Enter your email',
                            labelStyle: TextStyle(color: Colors.white),
                          ),
                          style: const TextStyle(color: Colors.white),
                        ),
                        TextFormField(
                          validator: _passwordValidator,
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
                        TextFormField(
                          validator: _passwordConfirmValidator,
                          controller: passwordConfirmController,
                          decoration: const InputDecoration(
                            enabledBorder: UnderlineInputBorder(
                              borderSide: BorderSide(color: Colors.white),
                            ),
                            labelText: 'Confirm your password',
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
                              onPressed: subscribe,
                              style: ElevatedButton.styleFrom(
                                backgroundColor: const Color(0xFF626af7),
                              ),
                              child: const Text("Create Account"),
                            ),
                          ),
                        ),
                      ],
                    ),
                  ),
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}
