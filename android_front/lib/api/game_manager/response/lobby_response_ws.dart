import 'package:miku/model/lobby/lobby_model.dart';

import '../../../model/game/game_model.dart';
import '../../../model/lobby/lobby_member_model.dart';
import '../../../mapper/game/game_response_mapper.dart';
import '../../../mapper/lobby/lobby_member_response_mapper.dart';

class LobbyResponseWS {
  final int id;
  final String code;
  final int gameId;
  final bool private;
  final List<LobbyMember> members;
  final Game game;

  LobbyResponseWS({
    required this.id,
    required this.code,
    required this.gameId,
    required this.private,
    required this.members,
    required this.game,
  });

  factory LobbyResponseWS.fromJson(Map<String, dynamic> json) {
    return LobbyResponseWS(
      id: json['id'],
      code: json['code'],
      gameId: json['game_id'],
      private: json['private'],
      members: (json['members'] as List<dynamic>)
          .map((member) => LobbyMemberResponseMapper.fromJson(member))
          .toList(),
      game: GameResponseMapper.fromJson(json['game']),
    );
  }

  Lobby toDomainLobby() {
    return Lobby(
      id: id,
      code: code,
      gameId: gameId,
      private: private,
      members: members,
      game: game,
    );
  }
}
