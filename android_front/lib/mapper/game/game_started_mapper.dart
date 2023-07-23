
import 'package:miku/model/game/game_started.dart';

import 'game_response_mapper.dart';
import '../lobby/lobby_member_response_mapper.dart';

class GameStartedMapper {

  static GameStarted fromJson(Map<String, dynamic> json) {
    return GameStarted(
      id: json['id'],
      members: (json['members'] as List<dynamic>).map((member) => LobbyMemberResponseMapper.fromJson(member)).toList(),
      game: GameResponseMapper.fromJson(json['game']),
    );
  }
}