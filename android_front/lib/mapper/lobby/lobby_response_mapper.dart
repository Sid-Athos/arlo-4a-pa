import '../game/game_response_mapper.dart';
import 'lobby_member_response_mapper.dart';
import '../../../../model/lobby/lobby_model.dart';

class LobbyResponseMapper {

  static Lobby fromJson(Map<String, dynamic> json) {
    return Lobby(
      id: json['id'],
      code: json['code'],
      gameId: json['game_id'],
      private: json['private'],
      members: (json['members'] as List<dynamic>).map((member) => LobbyMemberResponseMapper.fromJson(member)).toList(),
      game: GameResponseMapper.fromJson(json['game']),
    );
  }
}