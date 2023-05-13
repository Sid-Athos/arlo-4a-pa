import 'package:flutter/cupertino.dart';
import 'package:miku/model/lobby_member_model.dart';
import 'dart:developer' as developer;

class Lobby extends ChangeNotifier {

  int id;
  String code;
  int gameId;
  bool private;
  List<LobbyMember> members;

  Lobby({
    this.id = 0,
    this.code = "",
    this.gameId = 0,
    this.private = false,
    this.members = const [],
  });

  LobbyMember getHost() {
    return members.firstWhere((element) => element.isHost);
  }

  void update(Map<String, dynamic> json) {
    id = json['id'];
    code = json['code'];
    gameId = json['game_id'];
    private = json['private'];
    members = json['members'].map<LobbyMember>((member) => LobbyMember.fromJson(member)).toList();
    notifyListeners();
  }
}
