import 'package:flutter/material.dart';
import 'package:miku/api/user/api_user.dart';
import 'dart:developer' as developer;

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Flutter Demo',
      theme: ThemeData(
        primarySwatch: Colors.blue,
      ),
      home: const MyHomePage(),
      debugShowCheckedModeBanner: false,
    );
  }
}

class MyHomePage extends StatefulWidget {
  const MyHomePage({super.key});

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {

  final GlobalKey<FormState> _formKey = GlobalKey<FormState>();
  final emailController = TextEditingController();
  final passwordController = TextEditingController();

  void signUserIn() async {
    developer.log('log me', name: 'my.app.category');
    developer.log('log me', name: 'my.app.category');
    developer.log('log me', name: 'my.app.category');
    developer.log('log me', name: 'my.app.category');
    developer.log('log me', name: 'my.app.category');
    developer.log('log me', name: 'my.app.category');
    developer.log('log me', name: 'my.app.category');
    developer.log('log me', name: 'my.app.category');
    developer.log('log me', name: 'my.app.category');
    developer.log('log me', name: 'my.app.category');
    developer.log('log me', name: 'my.app.category');
    developer.log('log me', name: 'my.app.category');
    developer.log('log me', name: 'my.app.category');
    developer.log('log me', name: 'my.app.category');
    developer.log('log me', name: 'my.app.category');
    final session = await ApiUser().login(emailController.text, passwordController.text);
    developer.log('log me2', name: 'my.app.category');
    developer.log(session as String, name: 'my.app.category');
  }

  String? _textFieldValidator(String? value) {
    if (value == null || value.isEmpty) {
      return 'Saisissez un texte';
    }
    return null;
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: <Widget>[
            const Padding(
              padding: EdgeInsets.symmetric(horizontal: 40, vertical: 16),
              child: Text(
                "Login",
                style: TextStyle(fontSize: 40),
              ),
            ),
            Padding(
              padding: const EdgeInsets.symmetric(horizontal: 40, vertical: 16),
              child: TextFormField(
                validator: _textFieldValidator,
                controller: emailController,
                decoration: const InputDecoration(
                  border: UnderlineInputBorder(),
                  labelText: 'Enter your email',
                ),
              ),
            ),
            Padding(
              padding: const EdgeInsets.symmetric(horizontal: 40, vertical: 16),
              child: TextFormField(
                validator: _textFieldValidator,
                controller: passwordController,
                decoration: const InputDecoration(
                  border: UnderlineInputBorder(),
                  labelText: 'Enter your password',
                ),
                obscureText: true,
              ),
            ),
            ElevatedButton(
              onPressed: signUserIn,
              child: const Text("Log In"),
            ),
            const SizedBox(height: 50),
          ],
        ),
      ),
    );
  }
}
