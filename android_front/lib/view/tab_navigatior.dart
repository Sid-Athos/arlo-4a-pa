import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:miku/view/game_list_view.dart';

import 'home_view.dart';
import 'lobby_list_view.dart';

class TabNavigatorRoutes {
  static const String root = '/';
}

class TabNavigator extends StatelessWidget {
  TabNavigator({required this.navigatorKey, required this.tabItem});

  final GlobalKey<NavigatorState> navigatorKey;
  final TabItem tabItem;

  Map<String, WidgetBuilder> _routeBuilders(BuildContext context,
      {int materialIndex: 500}) {
    return {
      TabNavigatorRoutes.root: (context) => Container(
            alignment: Alignment.center,
            child: ElevatedButton(
              onPressed: () {
                Navigator.of(context).push(MaterialPageRoute(
                    builder: (context) => const LobbyScreen()));
              },
              child: const Text('PUSH'),
            ),
          ),
    };
  }

  @override
  Widget build(BuildContext context) {
    var routeBuilders = _routeBuilders(context);

    switch (tabItem) {
      case TabItem.friends:
        return Navigator(
            key: navigatorKey,
            initialRoute: TabNavigatorRoutes.root,
            onGenerateRoute: (routeSettings) {
              return MaterialPageRoute(
                  builder: (context) =>
                      routeBuilders[routeSettings.name]!(context));
            });
      case TabItem.game:
        return Navigator(
            key: navigatorKey,
            initialRoute: TabNavigatorRoutes.root,
            onGenerateRoute: (routeSettings) {
              return MaterialPageRoute(
                  builder: (context) =>
                      routeBuilders[routeSettings.name]!(context));
            });
      case TabItem.profile:
        return Navigator(
            key: navigatorKey,
            initialRoute: TabNavigatorRoutes.root,
            onGenerateRoute: (routeSettings) {
              return MaterialPageRoute(
                  builder: (context) =>
                      routeBuilders[routeSettings.name]!(context));
            });
    }
  }
}
