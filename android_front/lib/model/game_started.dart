import 'package:miku/model/lobby_member_model.dart';

import 'game_model.dart';

class GameStarted {

  int id;
  List<LobbyMember> members;
  Game game;

  GameStarted({
    required this.id,
    required this.members,
    required this.game,
  });
}
