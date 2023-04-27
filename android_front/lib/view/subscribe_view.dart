import 'package:flutter/material.dart';
import 'dart:developer' as developer;

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

  void subscribe() async {
    if (_formKey.currentState!.validate()) {
      developer.log("Subscribe");
    }
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
    return Scaffold(
      body: Center(
        child: Container(
          decoration: const BoxDecoration(
            image: DecorationImage(
                image: AssetImage("assets/img/background.png"),
                fit: BoxFit.cover
            ),
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
                        padding: const EdgeInsets.only(top: 40),
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
    );
  }
}
