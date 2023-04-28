import 'package:flutter/material.dart';
import 'package:miku/view/login_view.dart';
import 'package:shared_preferences/shared_preferences.dart';

import 'package:miku/view/home_view.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();

  SharedPreferences prefs = await SharedPreferences.getInstance();
  if (prefs.containsKey('login_token')) {
    runApp(const AppLogged());
  } else {
    runApp(const AppUnLogged());
  }
}

class AppUnLogged extends StatelessWidget {
  const AppUnLogged({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Miku',
      theme: ThemeData(
        primarySwatch: Colors.blue,
      ),
      home: const LoginPage(),
      debugShowCheckedModeBanner: false,
    );
  }
}

class AppLogged extends StatelessWidget {
  const AppLogged({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Miku',
      theme: ThemeData(
        primarySwatch: Colors.blue,
      ),
      home: const HomeView(),
      debugShowCheckedModeBanner: false,
    );
  }
}
