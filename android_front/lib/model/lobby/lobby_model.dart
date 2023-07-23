import 'package:flutter/cupertino.dart';
import 'package:miku/model/lobby/lobby_member_model.dart';
import 'dart:developer' as developer;

import '../game/game_model.dart';
import '../../mapper/game/game_response_mapper.dart';

class Lobby extends ChangeNotifier {

  int id = 0;
  String code = "";
  int gameId = 0;
  bool private = false;
  List<LobbyMember> members = const [];
  Game game = Game(id: 0, name: "", description: "", minPlayers: 0, maxPlayers: 0);

  Lobby({
    required this.id,
    required this.code,
    required this.gameId,
    required this.private,
    required this.members,
    required this.game,
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
    game = GameResponseMapper.fromJson(json['game']);
    notifyListeners();
  }
}
