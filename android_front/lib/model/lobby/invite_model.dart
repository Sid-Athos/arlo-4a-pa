import 'package:miku/model/lobby/lobby_model.dart';
import 'package:miku/model/user/user_model.dart';

class Invite {

  User from;
  User to;
  Lobby lobby;

  Invite({
    required this.from,
    required this.to,
    required this.lobby
  });

}