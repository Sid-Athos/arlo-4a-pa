
import 'package:miku/model/lobby_member_model.dart';

class Lobby {

  int id;
  String code;
  int gameId;
  bool private;
  List<LobbyMember> members;

  Lobby({
    required this.id,
    required this.code,
    required this.gameId,
    required this.private,
    required this.members,
  });
}
